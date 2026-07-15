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
        // Dynamic title per ui-interaction-spec §2.1
        if let Some(ref path) = self.current_file_path {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            format!("PPT_Agent_Rust — {}", name)
        } else {
            "PPT_Agent_Rust - PDF to Markdown Converter".to_string()
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        crate::update_handler::handle_message(self, message)
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
                &self.config,
            ),
            Route::Settings => settings_tab::view(&self.config, &self.api_key),
        };

        column![nav, content].into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let rx_mutex = self.event_receiver.clone();
        Subscription::from_recipe(crate::subscriptions::job_events::JobEventsRecipe { rx_mutex })
    }
}
