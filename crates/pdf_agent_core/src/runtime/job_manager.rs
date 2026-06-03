use crate::context::pipeline_context::PipelineContext;
use crate::pipeline::cancel_token::CancelToken;
use crate::pipeline::converter::PdfConverter;
use crate::pipeline::job_event::JobEvent;
use crate::providers::traits::DocumentProvider;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

pub struct ActiveJob {
    pub cancel_token: CancelToken,
}

pub struct JobManager {
    active_jobs: Arc<Mutex<HashMap<String, ActiveJob>>>,
    converter: Arc<PdfConverter>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            active_jobs: Arc::new(Mutex::new(HashMap::new())),
            converter: Arc::new(PdfConverter::new()),
        }
    }

    pub fn start_job(
        &self,
        job_id: String,
        provider: Arc<dyn DocumentProvider>,
        ctx: Arc<PipelineContext>,
    ) -> UnboundedReceiver<JobEvent> {
        let (tx, rx) = unbounded_channel();
        let cancel_token = CancelToken::new();

        {
            let mut active = self.active_jobs.lock().unwrap();
            active.insert(
                job_id.clone(),
                ActiveJob {
                    cancel_token: cancel_token.clone(),
                },
            );
        }

        let active_jobs_clone = self.active_jobs.clone();
        let converter_clone = self.converter.clone();
        let job_id_clone = job_id.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let _ = tx_clone.send(JobEvent::Started {
                job_id: job_id_clone.clone(),
            });

            match converter_clone
                .convert(&job_id_clone, &*provider, &*ctx, &cancel_token, Some(tx_clone.clone()))
                .await
            {
                Ok((markdown, document)) => {
                    let _ = tx_clone.send(JobEvent::Finished {
                        job_id: job_id_clone.clone(),
                        markdown,
                        document,
                    });
                }
                Err(e) => {
                    let _ = tx_clone.send(JobEvent::Failed {
                        job_id: job_id_clone.clone(),
                        error: e.to_string(),
                    });
                }
            }

            let mut active = active_jobs_clone.lock().unwrap();
            active.remove(&job_id_clone);
        });

        rx
    }

    pub fn cancel_job(&self, job_id: &str) -> bool {
        let mut active = self.active_jobs.lock().unwrap();
        if let Some(job) = active.remove(job_id) {
            job.cancel_token.cancel();
            true
        } else {
            false
        }
    }
}

impl Default for JobManager {
    fn default() -> Self {
        Self::new()
    }
}
