use std::{collections::HashMap, sync::Arc};

use tiny_http::{Server, StatusCode};

use crate::{
    error::Error,
    response::Response,
    service::{Command, Param, Service, System},
    Request,
};

/// Main application responsible for handling all net requests, resources, threading, and routing
/// this should be the base of any application made on simple-http
pub struct Application<Data = ()>
where
    Data: Send + Sync,
{
    root: Arc<Service<Data>>,
    server: Server,
    data: Arc<Data>,
}

impl<Data> Application<Data>
where
    Data: Send + Sync + 'static,
{
    /// Constructs a new instance of an application given an address structured as: `ip:port`, a
    /// root node, and some initial data.
    ///
    /// ```rust
    /// Application::new("0.0.0.0:80", Node::root(), ())
    /// ```
    pub fn new(
        addr: &str,
        root: Service<Data>,
        data: Data,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Self {
            root: Arc::new(root),
            server: Server::http(addr)?,
            data: Arc::new(data),
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
            let context_clone = self.data.clone();

            std::thread::spawn(move || {
                let mut url_values = HashMap::<String, Vec<String>>::new();
                let mut services = Vec::<&System<Data>>::new();

                let mut cur_node = root_clone.as_ref();

                let mut segment_iter = tiny_request.url().split_terminator("/").skip(1);

                'segment_iter: loop {
                    if let Some(callback) = cur_node.systems() {
                        services.push(callback)
                    }

                    match cur_node.param() {
                        Param::CollectExact(name, amount) => {
                            let mut collected_segments = Vec::new();
                            for _ in 0..*amount {
                                let Some(segment) = segment_iter.next() else {
                                    break 'segment_iter;
                                };

                                collected_segments.push(segment.to_string());
                            }

                            url_values.insert(name.to_owned(), collected_segments);
                        }
                        Param::CollectMaybe(name, amount) => {
                            let mut collected_segments = Vec::new();
                            for _ in 0..*amount {
                                let Some(segment) = segment_iter.next() else {
                                   break;
                                };

                                collected_segments.push(segment.to_string());
                            }

                            url_values.insert(name.to_owned(), collected_segments);
                        }
                        Param::CollectAll(name) => {
                            let collected_segments =
                                segment_iter.map(|s| s.to_string()).collect::<Vec<String>>();
                            url_values.insert(name.to_owned(), collected_segments);

                            break 'segment_iter;
                        }
                        Param::None => {}
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

                let mut request = Request::from_request(&mut tiny_request, url_values);

                for service in services {
                    let command = service.call(&mut request, context_clone.as_ref());

                    match command {
                        Command::Respond(response) => {
                            let _ = tiny_request.respond(response.into());
                            return;
                        }
                        Command::Upgrade() => {
                            todo!("");
                            //drop(request);
                            //let ws = tiny_request.upgrade(
                            //"websocket",
                            //websocket_service
                            //.take_initial_response()
                            //.expect("Response already taken")
                            //.into(),
                            //);

                            //websocket_service.run(context_clone.clone(), ws);
                            return;
                        }
                        Command::None => continue,
                    }
                }

                let _ = tiny_request.respond(Response::empty(StatusCode(500)).into());
            });
        }
    }
}
