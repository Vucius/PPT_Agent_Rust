use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ServiceRegistry {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: Send + Sync + 'static>(&mut self, service: Arc<T>) {
        self.services.insert(TypeId::of::<T>(), service);
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|s| s.clone().downcast::<T>().ok())
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
