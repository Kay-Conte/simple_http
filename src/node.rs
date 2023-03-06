use std::collections::HashMap;

use crate::{Request, Response};

/// Service callback type used by application

pub type ServiceFn = fn(&Request) -> Option<Response>;

pub struct Service {
    services: Vec<ServiceFn>,
}

impl Service {
    pub fn new(services: Vec<ServiceFn>) -> Self {
        Self {
            services,       
        }
    }

    pub fn single(service: ServiceFn) -> Self {
        Self {
            services: vec![service],
        }

    }

    pub fn call(&self, request: &Request) -> Option<crate::response::Response> {
        None
    }
}

impl From<ServiceFn> for Service {
    fn from(value: ServiceFn) -> Self {
        Self::single(value)
    }
}

/// Simple node type, represents a portion of an http url route
pub struct Node {
    path: String,
    param: Option<String>,
    service: Option<Service>,
    children: HashMap<String, Box<Node>>,
}

impl Node {
    pub fn new(
        path: impl Into<String>,
        service: Option<ServiceFn>,
        param: Option<String>,
    ) -> Self {
        Self {
            path: path.into(),
            param,
            service: service.map(|inner| inner.into()),
            children: HashMap::new(),
        }
    }

    /// Constructs root node for application, all applications should start with a root.
    pub fn root() -> Self {
        Self::new("root".to_string(), None, None)
    }

    // Special constructors //

    /// Constructs a node with no additional functionality
    pub fn new_path(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            param: None,
            service: None,
            children: HashMap::new(),
        }
    }

    /// Constructs a node with only a service
    pub fn new_service(path: impl Into<String>, callback: impl Into<Service>) -> Self {
        Self {
            path: path.into(),
            param: None,
            service: Some(callback.into()),
            children: HashMap::new(),
        }
    }

    /// Constructs a parameter type node used for collecting url values
    pub fn new_param(path: impl Into<String>, name: String) -> Self {
        Self {
            path: path.into(),
            param: Some(name),
            service: None,
            children: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, service: impl Into<Service>) {
        todo!()
    }

    pub fn get_child(&self, path: &str) -> Option<&Box<Node>> {
        self.children.get(path)
    }

    pub fn insert_child(mut self, child: Node) -> Self {
        self.children.insert(child.path.clone(), child.into());
        self
    }

    pub fn service(&self) -> &Option<Service> {
        &self.service
    }

    pub fn param(&self) -> &Option<String> {
        &self.param
    }
}
