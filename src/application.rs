use std::{collections::HashMap, sync::Arc};

use tiny_http::{Server, StatusCode};
use tokio::runtime::Runtime;

use crate::{
    error::Error,
    node::{Node, Service},
    response::Response,
};

/// Main application responsible for handling all net requests, resources, threading, and routing
pub struct Application {
    root: Arc<Node>,
    server: Server,
}

impl Application {
    pub fn new(
        addr: &str,
        root: Node,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Self {
            root: Arc::new(root),
            server: Server::http(addr)?,
        })
    }

    pub fn run(self) -> Result<(), Error> {
        println!("Starting runtime");

        let rt = Runtime::new().map_err(|_| Error::FailedToInitializeRuntime)?;

        //Enter tokio runtime context allowing thread spawning
        // rt.enter();

        //Spawn tokio runtime

        println!("Starting main thread");

        let res = rt.block_on(async move {
            println!("Creating stream");

            'server: loop {
                println!("looping");

                // TODO recv is blocking move to non blocking api
                let Ok(tiny_request) = self.server.recv() else {
                    println!("Server recv ended");
                    return Err(Error::ServerClosed);
                };

                //TODO wrap root node in ARC and spawn thread here

                let mut url_values = HashMap::<String, String>::new();
                let mut services = Vec::<&Service>::new();

                let mut cur_node = self.root.as_ref();

                if let Some(service) = cur_node.service() {
                    services.push(service);
                }

                let mut segment_iter = tiny_request.url().split("/").skip(1);

                while let Some(segment) = segment_iter.next() {
                    let Some(child) = cur_node.get_child(segment) else {
                        // invalid path
                        let response = Response::empty(StatusCode(404));

                        let _ = tiny_request.respond(response.into());

                        continue 'server;
                    };

                    if let Some(callback) = child.service() {
                        services.push(callback)
                    }

                    if let Some(param) = child.param() {
                        if let Some(url_value) = segment_iter.next() {
                            url_values.insert(param.clone(), url_value.to_string());
                        } else {
                            // invalid path
                            let response = Response::empty(StatusCode(404));

                            let _ = tiny_request.respond(response.into());

                            continue 'server;
                        }
                    }

                    cur_node = child;
                }

                let request = crate::Request::from_request(&tiny_request, url_values);

                for service in services {
                    if let Some(response) = service.call(&request) {
                        let _ = tiny_request.respond(response.into());
                        continue 'server;
                    }
                }

                let _ = tiny_request.respond(Response::empty(StatusCode(500)).into());
            }
        });

        res
    }
}
