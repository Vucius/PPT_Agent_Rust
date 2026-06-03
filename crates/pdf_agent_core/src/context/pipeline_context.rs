use crate::config::PipelineConfig;
use crate::context::service_registry::ServiceRegistry;

pub struct PipelineContext {
    pub config: PipelineConfig,
    pub registry: ServiceRegistry,
}

impl PipelineContext {
    pub fn new(config: PipelineConfig, registry: ServiceRegistry) -> Self {
        Self { config, registry }
    }
}
