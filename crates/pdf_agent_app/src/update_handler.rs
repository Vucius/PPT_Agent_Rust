use crate::app::App;
use crate::message::Message;
use crate::state::MainState;
use iced::Command;

/// Maximum number of versions kept in the undo/redo history stack.
/// Defined in ui-interaction-spec.md §6.3 — "最大深度: 20 版本".
const MAX_HISTORY_DEPTH: usize = 20;

/// Central message handler — delegates from `App::update`.
///
/// Every `Message` variant is handled here to keep `app.rs` as a thin
/// Application-trait skeleton (< 200 lines).
pub fn handle_message(app: &mut App, message: Message) -> Command<Message> {
    match message {
        // ─── 路由 ───
        Message::TabChanged(route) => {
            app.route = route;
            Command::none()
        }

        // ─── PDF 文件操作 ───
        Message::OpenFileClicked => crate::commands::file_commands::pick_file_cmd(),

        Message::PdfLoaded { path, page_count } => {
            app.current_file_path = Some(path.clone());
            app.main_state = MainState::PdfLoaded {
                file_path: path.clone(),
                page_count,
            };
            app.current_page_index = 0;
            app.total_pages = page_count;
            app.rendered_page_image = None;
            app.is_loading_image = true;
            app.image_error = None;
            crate::commands::file_commands::render_page_cmd(path, 0, 150)
        }

        Message::PdfLoadFailed(err) => {
            app.main_state = MainState::Failed {
                file_path: None,
                error: err,
            };
            Command::none()
        }

        Message::PageChanged(page_index) => {
            if let Some(ref path) = app.current_file_path {
                if page_index < app.total_pages {
                    app.current_page_index = page_index;
                    app.is_loading_image = true;
                    app.image_error = None;
                    return crate::commands::file_commands::render_page_cmd(
                        path.clone(),
                        page_index,
                        150,
                    );
                }
            }
            Command::none()
        }

        Message::PageImageLoaded { page_index, image } => {
            if page_index == app.current_page_index {
                app.rendered_page_image = Some(image);
                app.is_loading_image = false;
            }
            Command::none()
        }

        Message::PageImageLoadFailed(err) => {
            app.is_loading_image = false;
            app.image_error = Some(err);
            Command::none()
        }

        // ─── 转换流程 ───
        Message::ConvertClicked => {
            if let Some(ref file_path) = app.current_file_path {
                let job_id = format!(
                    "job_{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                );
                app.current_job_id = Some(job_id.clone());

                return crate::commands::conversion_commands::start_conversion_cmd(
                    job_id,
                    file_path.clone(),
                    app.config.clone(),
                    app.api_key.clone(),
                    app.job_manager.clone(),
                    app.event_receiver.clone(),
                );
            }
            Command::none()
        }

        Message::ConvertJobStarted(job_id) => {
            if let Some(ref path) = app.current_file_path {
                app.main_state = MainState::Converting {
                    file_path: path.clone(),
                    progress: pdf_agent_core::pipeline::job_event::JobProgress {
                        job_id,
                        stage: pdf_agent_core::pipeline::stage::PipelineStage::Idle,
                        current_page: 0,
                        total_pages: 0,
                        elapsed_seconds: 0.0,
                    },
                };
            }
            Command::none()
        }

        Message::JobProgressUpdate(progress) => {
            if let Some(ref path) = app.current_file_path {
                app.main_state = MainState::Converting {
                    file_path: path.clone(),
                    progress,
                };
            }
            Command::none()
        }

        Message::JobFinished {
            markdown, document, ..
        } => {
            if let Some(ref path) = app.current_file_path {
                app.main_state = MainState::Converted {
                    file_path: path.clone(),
                    markdown: markdown.clone(),
                    document: document.clone(),
                };
                app.history = vec![(document, markdown)];
                app.history_index = 0;
                app.selected_block_id = None;
                app.diff_mode = false;
                app.patch_preview = None;
            }
            Command::none()
        }

        Message::JobFailed { error, .. } => {
            app.main_state = MainState::Failed {
                file_path: app.current_file_path.clone(),
                error,
            };
            Command::none()
        }

        Message::CancelClicked => {
            if let Some(ref job_id) = app.current_job_id {
                app.job_manager.cancel_job(job_id);
            }
            if let Some(ref path) = app.current_file_path {
                app.main_state = MainState::PdfLoaded {
                    file_path: path.clone(),
                    page_count: 0,
                };
            } else {
                app.main_state = MainState::Empty;
            }
            Command::none()
        }

        // ─── LLM 反馈 ───
        Message::FeedbackInputChanged(input) => {
            app.feedback_input = input;
            Command::none()
        }

        Message::SubmitFeedbackClicked => {
            if let Some(ref block_id) = app.selected_block_id {
                if let MainState::Converted { document, .. } = &app.main_state {
                    let cmd = crate::commands::llm_commands::submit_feedback_cmd(
                        document.clone(),
                        block_id.clone(),
                        app.feedback_input.clone(),
                        app.config.clone(),
                        app.api_key.clone(),
                    );
                    app.feedback_input.clear();
                    return cmd;
                }
            }
            Command::none()
        }

        Message::LlmFeedbackResult(result) => {
            match result {
                Ok((new_markdown, document_clone)) => {
                    app.patch_preview = Some(new_markdown);
                    app.diff_mode = true;
                    app.history.truncate(app.history_index + 1);
                    app.history
                        .push((document_clone, app.patch_preview.clone().unwrap()));

                    // Enforce max history depth (ui-interaction-spec §6.3)
                    while app.history.len() > MAX_HISTORY_DEPTH {
                        app.history.remove(0);
                        if app.history_index > 0 {
                            app.history_index -= 1;
                        }
                    }
                }
                Err(e) => {
                    app.image_error = Some(format!("LLM Feedback error: {}", e));
                }
            }
            Command::none()
        }

        Message::BlockSelected(id_opt) => {
            app.selected_block_id = id_opt;
            Command::none()
        }

        // ─── Diff 操作 ───
        Message::AcceptPatchClicked => {
            app.diff_mode = false;
            app.patch_preview = None;
            app.history_index = app.history.len() - 1;
            if let Some(ref path) = app.current_file_path {
                let (doc, md) = &app.history[app.history_index];
                app.main_state = MainState::Converted {
                    file_path: path.clone(),
                    markdown: md.clone(),
                    document: doc.clone(),
                };
            }
            Command::none()
        }

        Message::RejectPatchClicked => {
            app.diff_mode = false;
            app.patch_preview = None;
            app.history.truncate(app.history_index + 1);
            Command::none()
        }

        // ─── 版本历史 ───
        Message::UndoClicked => {
            if app.history_index > 0 {
                app.history_index -= 1;
                if let Some(ref path) = app.current_file_path {
                    let (doc, md) = &app.history[app.history_index];
                    app.main_state = MainState::Converted {
                        file_path: path.clone(),
                        markdown: md.clone(),
                        document: doc.clone(),
                    };
                }
            }
            Command::none()
        }

        Message::RedoClicked => {
            if app.history_index + 1 < app.history.len() {
                app.history_index += 1;
                if let Some(ref path) = app.current_file_path {
                    let (doc, md) = &app.history[app.history_index];
                    app.main_state = MainState::Converted {
                        file_path: path.clone(),
                        markdown: md.clone(),
                        document: doc.clone(),
                    };
                }
            }
            Command::none()
        }

        // ─── Settings ───
        Message::OcrModeChanged(mode) => {
            app.config.ocr_mode = mode;
            Command::none()
        }

        Message::OutputFormatChanged(fmt) => {
            app.config.output_format = fmt;
            Command::none()
        }

        Message::LlmProviderChanged(prov) => {
            app.config.llm.provider = prov;
            Command::none()
        }

        Message::LlmModelChanged(model) => {
            app.config.llm.model_name = model;
            Command::none()
        }

        Message::LlmBaseUrlChanged(url) => {
            app.config.llm.base_url = url;
            Command::none()
        }

        Message::LlmKeyChanged(key) => {
            app.api_key = key;
            Command::none()
        }

        Message::LlmLimitChanged(lim) => {
            if let Ok(l) = lim.parse::<i64>() {
                app.config.llm.daily_limit = l;
            }
            Command::none()
        }

        Message::SaveSettingsClicked => {
            crate::commands::settings_commands::save_settings_cmd(
                &app.config.llm.provider,
                &app.api_key,
            );
            Command::none()
        }

        // ─── 文件导出 ───
        Message::ExportMarkdownClicked => {
            if let MainState::Converted {
                markdown, file_path, ..
            } = &app.main_state
            {
                let mut default_filename = file_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "document".to_string());

                if default_filename.ends_with(".pdf") {
                    default_filename = default_filename.replace(".pdf", ".md");
                } else {
                    default_filename.push_str(".md");
                }

                return crate::commands::file_commands::export_markdown_cmd(
                    markdown.clone(),
                    default_filename,
                );
            }
            Command::none()
        }

        Message::ExportJsonClicked => {
            if let MainState::Converted {
                document, file_path, ..
            } = &app.main_state
            {
                let mut default_filename = file_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "document".to_string());

                if default_filename.ends_with(".pdf") {
                    default_filename = default_filename.replace(".pdf", ".json");
                } else {
                    default_filename.push_str(".json");
                }

                return crate::commands::file_commands::export_json_cmd(
                    document.clone(),
                    default_filename,
                );
            }
            Command::none()
        }

        Message::ExportCompleted(result) => {
            match result {
                Ok(path) => {
                    println!("Exported successfully to {}", path);
                }
                Err(e) => {
                    app.image_error = Some(format!("Export failed: {}", e));
                }
            }
            Command::none()
        }
    }
}
