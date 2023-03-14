use std::collections::HashMap;

use crate::{websocket::WebsocketServiceExport, request::Request, response::Response};

/// Service callback type used by application
pub type SystemFn<Data> = fn(&mut Request, &Data) -> Command<Data>;

/// Describes the action of a `System`
pub enum Command<Data> {
    /// Upgrade current connection to a websocket. This assumes the client is already trying to connect over Ws.
    Upgrade(Response, Box<dyn WebsocketServiceExport<Data>>),

    /// Respond to request and don't step further services in tree.
    Respond(Response),

    /// Do nothing, move on to next service in tree, If no further services, return 404.
    None,
}

impl<Data> Command<Data> {
    /// Checks if command is `None`
    pub fn is_none(&self) -> bool {
        match self {
            Command::None => true,
            _ => false,
        }
    }
}

/// Systems are thin wrappers over a list of functions normally associated with a `Service`. Having
/// multiple systems allows easy reuse of common middleware responsible for gathering information
/// or parsing data.
pub struct System<Data> {
    collection: Vec<SystemFn<Data>>,
}

impl<Data> System<Data> {
    /// System constructor
    pub fn new(services: Vec<SystemFn<Data>>) -> Self {
        Self {
            collection: services,
        }
    }

    /// Constructs a system with only one `SystemFn`
    pub fn single(service: SystemFn<Data>) -> Self {
        Self {
            collection: vec![service],
        }
    }

    /// Calls a systems underlying functions in order
    pub fn call(&self, request: &mut Request, data: &Data) -> Command<Data> {
        for system in self.collection.iter() {
            let res = system(request, data);

            if !res.is_none() {
                return res;
            }
        }

        Command::None
    }
}

impl<Data> From<SystemFn<Data>> for System<Data> {
    fn from(value: SystemFn<Data>) -> Self {
        Self::single(value)
    }
}

/// Describes which url values to collect after a Service.
pub enum Param {
    /// Collect all url segments after attached Service into url value map.
    CollectAll(String),

    /// Collect some number of url segments into url value map. Run system even if not enough
    /// values.
    CollectMaybe(String, usize),

    /// Collect some number of url segments into url value map. Run system only if enough values
    CollectExact(String, usize),

    /// Don't Collect any url segments.
    None,
}

/// Simple node type, represents a portion of an http url route
pub struct Service<Data> {
    path: String,
    param: Param,
    systems: Option<System<Data>>,
    children: HashMap<String, Box<Service<Data>>>,
}

impl<Data> Service<Data> {
    /// Construct a new service
    pub fn new(path: impl Into<String>, service: Option<SystemFn<Data>>, param: Param) -> Self {
        Self {
            path: path.into(),
            param,
            systems: service.map(|inner| inner.into()),
            children: HashMap::new(),
        }
    }

    /// Pass a closure a mutable reference to self. This is good for creating Tree like structures
    /// without binding multiple variables.
    pub fn fold(mut self, callback: fn(&mut Self)) -> Self {
        callback(&mut self);

        self
    }

    /// Constructs root node for application, all applications should start with a root.
    pub fn root() -> Self {
        Self::new("root".to_string(), None, Param::None)
    }

    /// Constructs a node with no additional functionality.
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            param: Param::None,
            systems: None,
            children: HashMap::new(),
        }
    }

    /// Constructs a Service with a System.
    pub fn with_system(path: impl Into<String>, callback: impl Into<System<Data>>) -> Self {
        Self {
            path: path.into(),
            param: Param::None,
            systems: Some(callback.into()),
            children: HashMap::new(),
        }
    }

    /// Constructs a parameter type Service used for collecting url values.
    pub fn with_param(path: impl Into<String>, name: String) -> Self {
        Self {
            path: path.into(),
            param: Param::CollectExact(name, 1),
            systems: None,
            children: HashMap::new(),
        }
    }

    pub fn insert_system(mut self, system: System<Data>) -> Self {
        self.systems = Some(system);

        self
    }

    pub fn insert_param(mut self, param: Param) -> Self {
        self.param = param;

        self
    }

    pub fn get_child(&self, path: &str) -> Option<&Box<Service<Data>>> {
        self.children.get(path)
    }

    pub fn insert_child(&mut self, child: Service<Data>) {
        self.children.insert(child.path.clone(), child.into());
    }

    pub fn systems(&self) -> &Option<System<Data>> {
        &self.systems
    }

    pub fn param(&self) -> &Param {
        &self.param
    }
}
