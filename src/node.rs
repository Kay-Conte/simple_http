use std::collections::HashMap;

use crate::{Request, Response};

/// Service callback type used by application

pub type SystemFn = fn(&Request) -> Option<Response>;

/// Systems are thin wrappers over a list of functions normally associated with a `Service`. Having
/// multiple systems allows easy reuse of common middleware responsible for gathering information
/// or parsing data.
pub struct System {
    collection: Vec<SystemFn>,
}

impl System {
    /// System constructor
    pub fn new(services: Vec<SystemFn>) -> Self {
        Self {
            collection: services,
        }
    }

    /// Constructs a system with only one `SystemFn`
    pub fn single(service: SystemFn) -> Self {
        Self {
            collection: vec![service],
        }
    }

    /// Calls a systems underlying functions in order
    pub fn call(&self, request: &Request) -> Option<crate::response::Response> {
        for system in self.collection.iter() {
            let res = system(request);

            if res.is_some() {
                return res;
            }
        }

        None
    }
}

impl From<SystemFn> for System {
    fn from(value: SystemFn) -> Self {
        Self::single(value)
    }
}

/// Simple node type, represents a portion of an http url route
pub struct Service {
    path: String,
    param: Option<String>,
    systems: Option<System>,
    children: HashMap<String, Box<Service>>,
}

impl Service {
    pub fn new(path: impl Into<String>, service: Option<SystemFn>, param: Option<String>) -> Self {
        Self {
            path: path.into(),
            param,
            systems: service.map(|inner| inner.into()),
            children: HashMap::new(),
        }
    }

    pub fn fold(mut self, callback: fn(&mut Self)) -> Self {
        callback(&mut self);

        self
    }

    /// Constructs root node for application, all applications should start with a root.
    pub fn root() -> Self {
        Self::new("root".to_string(), None, None)
    }

    /// Constructs a node with no additional functionality
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            param: None,
            systems: None,
            children: HashMap::new(),
        }
    }

    /// Constructs a node with only a service
    pub fn with_system(path: impl Into<String>, callback: impl Into<System>) -> Self {
        Self {
            path: path.into(),
            param: None,
            systems: Some(callback.into()),
            children: HashMap::new(),
        }
    }

    /// Constructs a parameter type node used for collecting url values
    pub fn with_param(path: impl Into<String>, name: String) -> Self {
        Self {
            path: path.into(),
            param: Some(name),
            systems: None,
            children: HashMap::new(),
        }
    }

    pub fn add_system(mut self, system: System) -> Self {
        self.systems = Some(system);

        self
    }

    pub fn add_param(mut self, name: String) -> Self {
        self.param = Some(name);

        self
    }

    pub fn get_child(&self, path: &str) -> Option<&Box<Service>> {
        self.children.get(path)
    }

    pub fn insert_child(&mut self, child: Service) {
        self.children.insert(child.path.clone(), child.into());
    }

    pub fn systems(&self) -> &Option<System> {
        &self.systems
    }

    pub fn param(&self) -> &Option<String> {
        &self.param
    }
}
