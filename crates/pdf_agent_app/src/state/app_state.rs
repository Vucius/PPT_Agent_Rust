use crate::message::Route;
use crate::state::main_state::MainState;
use pdf_agent_core::runtime::JobManager;
use std::sync::Arc;

#[allow(dead_code)]
pub struct AppState {
    pub route: Route,
    pub main_state: MainState,
    pub feedback_input: String,
    pub job_manager: Arc<JobManager>,
    pub current_job_id: Option<String>,
}

#[allow(dead_code)]
impl AppState {
    pub fn new() -> Self {
        Self {
            route: Route::Main,
            main_state: MainState::Empty,
            feedback_input: String::new(),
            job_manager: Arc::new(JobManager::new()),
            current_job_id: None,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
