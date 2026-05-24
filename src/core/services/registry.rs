use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Base trait for all services that can be registered in the ServiceRegistry.
pub trait Service: Any + Send + Sync {}

/// A registry for managing global services in a dependency-injection-like manner.
#[derive(Clone, Default)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a service in the registry.
    pub fn register<S: Service>(&self, service: S) {
        let mut services = self.services.write().expect("Failed to lock services for writing");
        services.insert(TypeId::of::<S>(), Arc::new(service));
    }

    /// Retrieves a service from the registry.
    pub fn get<S: Service>(&self) -> Option<Arc<S>> {
        let services = self.services.read().expect("Failed to lock services for reading");
        services.get(&TypeId::of::<S>())
            .and_then(|arc| arc.clone().downcast::<S>().ok())
    }
}
