use crate::message::{Message, Route};
use crate::screens::{main_tab, settings_tab};
use crate::state::MainState;
use iced::widget::{button, column, row, Space};
use iced::{executor, Application, Command, Element, Length, Subscription, Theme};
use pdf_agent_core::pipeline::job_event::JobEvent;
use pdf_agent_core::runtime::JobManager;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct App {
    pub route: Route,
    pub main_state: MainState,
    pub feedback_input: String,
    pub job_manager: Arc<JobManager>,
    pub current_job_id: Option<String>,
    pub current_file_path: Option<PathBuf>,
    pub event_receiver: Arc<tokio::sync::Mutex<Option<UnboundedReceiver<JobEvent>>>>,
    pub current_page_index: usize,
    pub total_pages: usize,
    pub rendered_page_image: Option<pdf_agent_core::providers::traits::PageImage>,
    pub is_loading_image: bool,
    pub image_error: Option<String>,
    pub selected_block_id: Option<String>,
    pub diff_mode: bool,
    pub patch_preview: Option<String>,
    pub history: Vec<(pdf_agent_core::schema::document::Document, String)>,
    pub history_index: usize,
    pub config: pdf_agent_core::config::PipelineConfig,
    pub api_key: String,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                route: Route::Main,
                main_state: MainState::Empty,
                feedback_input: String::new(),
                job_manager: Arc::new(JobManager::new()),
                current_job_id: None,
                current_file_path: None,
                event_receiver: Arc::new(tokio::sync::Mutex::new(None)),
                current_page_index: 0,
                total_pages: 0,
                rendered_page_image: None,
                is_loading_image: false,
                image_error: None,
                selected_block_id: None,
                diff_mode: false,
                patch_preview: None,
                history: Vec::new(),
                history_index: 0,
                config: pdf_agent_core::config::PipelineConfig::default(),
                api_key: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "PPT_Agent_Rust - PDF to Markdown Converter".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TabChanged(route) => {
                self.route = route;
                Command::none()
            }
            Message::OpenFileClicked => Command::perform(
                async {
                    let file = rfd::AsyncFileDialog::new()
                        .add_filter("PDF files", &["pdf"])
                        .pick_file()
                        .await;
                    file.map(|f| f.path().to_path_buf())
                },
                |path_opt| match path_opt {
                    Some(path) => match pdf_agent_pdf::PdfProvider::open(&path) {
                        Ok(provider) => {
                            use pdf_agent_core::providers::traits::DocumentProvider;
                            let page_count = provider.page_count().unwrap_or(0);
                            Message::PdfLoaded { path, page_count }
                        }
                        Err(e) => Message::PdfLoadFailed(e.to_string()),
                    },
                    None => Message::PdfLoadFailed("No file selected".to_string()),
                },
            ),
            Message::PdfLoaded { path, page_count } => {
                self.current_file_path = Some(path.clone());
                self.main_state = MainState::PdfLoaded {
                    file_path: path.clone(),
                    page_count,
                };
                self.current_page_index = 0;
                self.total_pages = page_count;
                self.rendered_page_image = None;
                self.is_loading_image = true;
                self.image_error = None;

                let path_clone = path;
                Command::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            let provider = pdf_agent_pdf::PdfProvider::open(&path_clone)
                                .map_err(|e| pdf_agent_core::error::Error::Pdf(e.to_string()))?;
                            use pdf_agent_core::providers::traits::DocumentProvider;
                            provider.render_page(0, 150)
                        })
                        .await
                        .map_err(|e| e.to_string())?
                        .map_err(|e| e.to_string())
                    },
                    |result| match result {
                        Ok(img) => Message::PageImageLoaded { page_index: 0, image: img },
                        Err(e) => Message::PageImageLoadFailed(e),
                    }
                )
            }
            Message::PdfLoadFailed(err) => {
                self.main_state = MainState::Failed {
                    file_path: None,
                    error: err,
                };
                Command::none()
            }
            Message::ConvertClicked => {
                if let Some(ref file_path) = self.current_file_path {
                    let job_id = format!(
                        "job_{}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                    );
                    self.current_job_id = Some(job_id.clone());

                    match pdf_agent_pdf::PdfProvider::open(file_path) {
                        Ok(provider) => {
                            let provider_arc = Arc::new(provider);
                            let mut registry = pdf_agent_core::context::ServiceRegistry::new();
                            let ocr_predictor = Arc::new(pdf_agent_inference::predictors::OcrPredictor::new());
                            let ocr_service = Arc::new(pdf_agent_core::providers::traits::OcrService::new(ocr_predictor));
                            registry.register(ocr_service);

                            let layout_predictor = Arc::new(pdf_agent_inference::predictors::LayoutPredictor::new());
                            let layout_service = Arc::new(pdf_agent_core::providers::traits::LayoutService::new(layout_predictor));
                            registry.register(layout_service);

                            let llm_service: Arc<dyn pdf_agent_core::providers::traits::LlmService> = if self.config.llm.provider == "mock" {
                                Arc::new(pdf_agent_llm::MockLlmService)
                            } else {
                                Arc::new(pdf_agent_llm::OpenAiLlmService::new(
                                    self.api_key.clone(),
                                    self.config.llm.base_url.clone(),
                                    self.config.llm.model_name.clone(),
                                ))
                            };
                            let llm_service_wrapper = Arc::new(pdf_agent_core::providers::traits::LlmServiceWrapper::new(llm_service));
                            registry.register(llm_service_wrapper);

                            let ctx = Arc::new(pdf_agent_core::context::PipelineContext::new(
                                self.config.clone(),
                                registry,
                            ));

                            let rx = self
                                .job_manager
                                .start_job(job_id, provider_arc, ctx);

                            let rx_mutex = self.event_receiver.clone();
                            return Command::perform(
                                async move {
                                    let mut opt = rx_mutex.lock().await;
                                    *opt = Some(rx);
                                },
                                |_| Message::FeedbackInputChanged(String::new()),
                            );
                        }
                        Err(e) => {
                            self.main_state = MainState::Failed {
                                file_path: Some(file_path.clone()),
                                error: e.to_string(),
                            };
                        }
                    }
                }
                Command::none()
            }
            Message::ConvertJobStarted(job_id) => {
                if let Some(ref path) = self.current_file_path {
                    self.main_state = MainState::Converting {
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
                if let Some(ref path) = self.current_file_path {
                    self.main_state = MainState::Converting {
                        file_path: path.clone(),
                        progress,
                    };
                }
                Command::none()
            }
            Message::JobFinished { markdown, document, .. } => {
                if let Some(ref path) = self.current_file_path {
                    self.main_state = MainState::Converted {
                        file_path: path.clone(),
                        markdown: markdown.clone(),
                        document: document.clone(),
                    };
                    self.history = vec![(document, markdown)];
                    self.history_index = 0;
                    self.selected_block_id = None;
                    self.diff_mode = false;
                    self.patch_preview = None;
                }
                Command::none()
            }
            Message::JobFailed { error, .. } => {
                self.main_state = MainState::Failed {
                    file_path: self.current_file_path.clone(),
                    error,
                };
                Command::none()
            }
            Message::CancelClicked => {
                if let Some(ref job_id) = self.current_job_id {
                    self.job_manager.cancel_job(job_id);
                }
                if let Some(ref path) = self.current_file_path {
                    self.main_state = MainState::PdfLoaded {
                        file_path: path.clone(),
                        page_count: 0,
                    };
                } else {
                    self.main_state = MainState::Empty;
                }
                Command::none()
            }
            Message::FeedbackInputChanged(input) => {
                self.feedback_input = input;
                Command::none()
            }
            Message::SubmitFeedbackClicked => {
                if let Some(ref block_id) = self.selected_block_id {
                    if let MainState::Converted { document, .. } = &self.main_state {
                        if let Some((prev, target, next)) = document.find_block_with_context(block_id) {
                            let feedback = self.feedback_input.clone();
                            let api_key = self.api_key.clone();
                            let config = self.config.clone();
                            let mut document_clone = document.clone();
                            let block_id_clone = block_id.clone();
                            
                            let prev_text = prev.map(|b| b.text.clone()).unwrap_or_else(|| "[None]".to_string());
                            let target_text = target.text.clone();
                            let next_text = next.map(|b| b.text.clone()).unwrap_or_else(|| "[None]".to_string());

                            self.feedback_input.clear();
                            
                            return Command::perform(
                                async move {
                                    // 1. Build prompt
                                    let system_prompt = "You are an assistant correcting OCR and layout parsing errors in a document block. Output only the corrected text for the target block. Do not wrap in markdown code blocks unless the block itself is a code block.";
                                    let user_prompt = format!(
                                        "Context:\n- Previous Block: {}\n- Target Block: {}\n- Next Block: {}\n\nUser Feedback: {}\n\nCorrected Target Block text:",
                                        prev_text, target_text, next_text, feedback
                                    );
                                    
                                    // 2. Call LLM
                                    let llm_service: std::sync::Arc<dyn pdf_agent_core::providers::traits::LlmService> = if config.llm.provider == "mock" {
                                        std::sync::Arc::new(pdf_agent_llm::MockLlmService)
                                    } else {
                                        std::sync::Arc::new(pdf_agent_llm::OpenAiLlmService::new(
                                            api_key,
                                            config.llm.base_url.clone(),
                                            config.llm.model_name.clone(),
                                        ))
                                    };
                                    
                                    let request = pdf_agent_core::providers::traits::LlmRequest {
                                        system_prompt: Some(system_prompt.to_string()),
                                        user_prompt,
                                        temperature: Some(0.2),
                                        max_tokens: Some(1024),
                                        json_mode: false,
                                    };
                                    
                                    let result = llm_service.complete(request).await;
                                    match result {
                                        Ok(new_text) => {
                                            // Update document in memory
                                            document_clone.update_block_text(&block_id_clone, &new_text);
                                            
                                            // Regenerate markdown
                                            let renderer = pdf_agent_core::renderers::markdown_renderer::MarkdownRenderer;
                                            let ctx = pdf_agent_core::context::PipelineContext::new(
                                                config,
                                                pdf_agent_core::context::ServiceRegistry::new()
                                            );
                                            use pdf_agent_core::renderers::traits::DocumentRenderer;
                                            let new_markdown = renderer.render(&document_clone, &ctx)
                                                .map_err(|e| e.to_string())?;
                                                
                                            // Increment daily quota in DB
                                            if let Ok(db) = pdf_agent_storage::DbConnection::open("pdf_agent.db") {
                                                // Estimate days from epoch for simple unique date tracking
                                                let today = std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH)
                                                    .map(|d| format!("day_{}", d.as_secs() / 86400))
                                                    .unwrap_or_else(|_| "today".to_string());
                                                let estimated_tokens = (new_text.len() + system_prompt.len()) as i64 / 4;
                                                let _ = db.increment_tokens_used(&today, estimated_tokens, 50000);
                                            }
                                            
                                            Ok((new_markdown, document_clone))
                                        }
                                        Err(e) => Err(e.to_string())
                                    }
                                },
                                |res| Message::LlmFeedbackResult(res)
                            );
                        }
                    }
                }
                Command::none()
            }
            Message::LlmFeedbackResult(result) => {
                match result {
                    Ok((new_markdown, document_clone)) => {
                        self.patch_preview = Some(new_markdown);
                        self.diff_mode = true;
                        // Temp store in history to allow undo/redo once accepted
                        self.history.truncate(self.history_index + 1);
                        self.history.push((document_clone, self.patch_preview.clone().unwrap()));
                    }
                    Err(e) => {
                        self.image_error = Some(format!("LLM Feedback error: {}", e));
                    }
                }
                Command::none()
            }
            Message::BlockSelected(id_opt) => {
                self.selected_block_id = id_opt;
                Command::none()
            }
            Message::UndoClicked => {
                if self.history_index > 0 {
                    self.history_index -= 1;
                    if let Some(ref path) = self.current_file_path {
                        let (doc, md) = &self.history[self.history_index];
                        self.main_state = MainState::Converted {
                            file_path: path.clone(),
                            markdown: md.clone(),
                            document: doc.clone(),
                        };
                    }
                }
                Command::none()
            }
            Message::RedoClicked => {
                if self.history_index + 1 < self.history.len() {
                    self.history_index += 1;
                    if let Some(ref path) = self.current_file_path {
                        let (doc, md) = &self.history[self.history_index];
                        self.main_state = MainState::Converted {
                            file_path: path.clone(),
                            markdown: md.clone(),
                            document: doc.clone(),
                        };
                    }
                }
                Command::none()
            }
            Message::AcceptPatchClicked => {
                self.diff_mode = false;
                self.patch_preview = None;
                self.history_index = self.history.len() - 1;
                if let Some(ref path) = self.current_file_path {
                    let (doc, md) = &self.history[self.history_index];
                    self.main_state = MainState::Converted {
                        file_path: path.clone(),
                        markdown: md.clone(),
                        document: doc.clone(),
                    };
                }
                Command::none()
            }
            Message::RejectPatchClicked => {
                self.diff_mode = false;
                self.patch_preview = None;
                self.history.truncate(self.history_index + 1);
                Command::none()
            }
            Message::OcrModeChanged(mode) => {
                self.config.ocr_mode = mode;
                Command::none()
            }
            Message::OutputFormatChanged(fmt) => {
                self.config.output_format = fmt;
                Command::none()
            }
            Message::LlmProviderChanged(prov) => {
                self.config.llm.provider = prov;
                Command::none()
            }
            Message::LlmModelChanged(model) => {
                self.config.llm.model_name = model;
                Command::none()
            }
            Message::LlmBaseUrlChanged(url) => {
                self.config.llm.base_url = url;
                Command::none()
            }
            Message::LlmKeyChanged(key) => {
                self.api_key = key;
                Command::none()
            }
            Message::LlmLimitChanged(lim) => {
                if let Ok(l) = lim.parse::<i64>() {
                    self.config.llm.daily_limit = l;
                }
                Command::none()
            }
            Message::SaveSettingsClicked => {
                let _ = pdf_agent_storage::KeyringStore::new()
                    .set_api_key(&self.config.llm.provider, &self.api_key);
                Command::none()
            }
            Message::PageChanged(page_index) => {
                if let Some(ref path) = self.current_file_path {
                    if page_index < self.total_pages {
                        self.current_page_index = page_index;
                        self.is_loading_image = true;
                        self.image_error = None;
                        let path_clone = path.clone();
                        return Command::perform(
                            async move {
                                tokio::task::spawn_blocking(move || {
                                    let provider = pdf_agent_pdf::PdfProvider::open(&path_clone)
                                        .map_err(|e| pdf_agent_core::error::Error::Pdf(e.to_string()))?;
                                    use pdf_agent_core::providers::traits::DocumentProvider;
                                    provider.render_page(page_index, 150)
                                })
                                .await
                                .map_err(|e| e.to_string())?
                                .map_err(|e| e.to_string())
                            },
                            move |result| match result {
                                Ok(img) => Message::PageImageLoaded { page_index, image: img },
                                Err(e) => Message::PageImageLoadFailed(e),
                            }
                        );
                    }
                }
                Command::none()
            }
            Message::PageImageLoaded { page_index, image } => {
                if page_index == self.current_page_index {
                    self.rendered_page_image = Some(image);
                    self.is_loading_image = false;
                }
                Command::none()
            }
            Message::PageImageLoadFailed(err) => {
                self.is_loading_image = false;
                self.image_error = Some(err);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let nav = row![
            button("Workspace").on_press(Message::TabChanged(Route::Main)),
            Space::with_width(10),
            button("Settings").on_press(Message::TabChanged(Route::Settings)),
        ]
        .padding(10)
        .width(Length::Fill);

        let content: Element<'_, Self::Message> = match self.route {
            Route::Main => main_tab::view(
                &self.main_state,
                &self.feedback_input,
                self.current_page_index,
                self.total_pages,
                self.rendered_page_image.as_ref(),
                self.is_loading_image,
                self.image_error.as_deref(),
                self.selected_block_id.as_deref(),
                self.diff_mode,
                self.patch_preview.as_deref(),
                self.history_index,
                self.history.len(),
            ),
            Route::Settings => settings_tab::view(&self.config, &self.api_key),
        };

        column![nav, content].into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let rx_mutex = self.event_receiver.clone();
        Subscription::from_recipe(JobEventsRecipe { rx_mutex })
    }
}

struct JobEventsRecipe {
    rx_mutex: Arc<tokio::sync::Mutex<Option<UnboundedReceiver<JobEvent>>>>,
}

impl iced::advanced::subscription::Recipe for JobEventsRecipe {
    type Output = Message;

    fn hash(&self, state: &mut iced::advanced::Hasher) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: iced::advanced::subscription::EventStream,
    ) -> iced::futures::stream::BoxStream<'static, Self::Output> {
        let rx_mutex = self.rx_mutex.clone();
        Box::pin(async_stream::stream! {
            loop {
                let mut opt = rx_mutex.lock().await;
                if let Some(ref mut rx) = *opt {
                    match rx.try_recv() {
                        Ok(event) => {
                            let msg = match event {
                                JobEvent::Started { job_id } => Message::ConvertJobStarted(job_id),
                                JobEvent::Progress(progress) => Message::JobProgressUpdate(progress),
                                JobEvent::Finished { job_id, markdown, document } => Message::JobFinished { job_id, markdown, document },
                                JobEvent::Failed { job_id, error } => Message::JobFailed { job_id, error },
                            };
                            yield msg;
                        }
                        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                            // Yield back to executor
                        }
                        Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                            *opt = None;
                        }
                    }
                }
                drop(opt);
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
    }
}
