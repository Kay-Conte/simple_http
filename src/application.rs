use std::{collections::HashMap, sync::Arc};

use tiny_http::{Server, StatusCode};
use tokio::runtime::Runtime;

use crate::{
    error::Error,
    node::{Service, System},
    response::Response,
};

/// Main application responsible for handling all net requests, resources, threading, and routing
/// this should be the base of any application made on simple-http
pub struct Application {
    root: Arc<Service>,
    server: Server,
}

impl Application {
    /// Constructs a new instance of an application given an address structured as: `ip:port` and a
    /// root node.
    ///
    /// ```rust
    /// Application::new("0.0.0.0:80", Node::root())
    /// ```
    pub fn new(
        addr: &str,
        root: Service,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Self {
            root: Arc::new(root),
            server: Server::http(addr)?,
        })
    }

    /// Initialize main application loop. This method is blocking and will only return on close or
    /// error
    pub fn run(self) -> Result<(), Error> {
        loop {
            let Ok(mut tiny_request) = self.server.recv() else {
                    return Err(Error::ServerClosed);
                };

            let root_clone = self.root.clone();

            std::thread::spawn(move || {
                let mut url_values = HashMap::<String, String>::new();
                let mut services = Vec::<&System>::new();

                let mut cur_node = root_clone.as_ref();

                let mut segment_iter = tiny_request.url().split_terminator("/").skip(1);

                'segment_iter: loop {
                    if let Some(callback) = cur_node.systems() {
                        services.push(callback)
                    }

                    if let Some(param) = cur_node.param() {
                        if let Some(url_value) = segment_iter.next() {
                            url_values.insert(param.clone(), url_value.to_string());
                        }
                    }

                    let Some(segment) = segment_iter.next() else {
                        break 'segment_iter;
                    };

                    let Some(child) = cur_node.get_child(segment) else {
                        let response = Response::empty(StatusCode(404));

                        let _ = tiny_request.respond(response.into());

                        return;
                    };

                    cur_node = child;
                }

                let request = crate::Request::from_request(&mut tiny_request, url_values);

                for service in services {
                    if let Some(response) = service.call(&request) {
                        let _ = tiny_request.respond(response.into());
                        return;
                    }
                }

                let _ = tiny_request.respond(Response::empty(StatusCode(500)).into());
            });
        }
    }
}
