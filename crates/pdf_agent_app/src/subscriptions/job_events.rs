use crate::message::Message;
use iced::advanced::subscription::{EventStream, Recipe};
use iced::advanced::Hasher;
use iced::futures::stream::BoxStream;
use pdf_agent_core::pipeline::job_event::JobEvent;
use std::any::TypeId;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct JobEventsRecipe {
    pub rx_mutex: Arc<tokio::sync::Mutex<Option<UnboundedReceiver<JobEvent>>>>,
}

impl Recipe for JobEventsRecipe {
    type Output = Message;

    fn hash(&self, state: &mut Hasher) {
        TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: EventStream,
    ) -> BoxStream<'static, Self::Output> {
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
                        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
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
