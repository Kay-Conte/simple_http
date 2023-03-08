use std::fs::File;

use simple_http::{Application, Method, Request, Response, Service, StatusCode, System};

// This example should be run from the project root directory using `cargo run --example hello_world`

// A system that returns `None` does not produce a response and simply acts as middleware. Services
// may have multiple systems and they will always be executed in order. A system that returns
// `Some(...)` will stop the task and produce a response to the request.

fn root(request: &Request) -> Option<Response> {
    let root_path = std::env::current_dir()
        .expect("Failed to get working directory")
        .join("examples/html");

    let Some(target) = request.get_url_value("file") else {
        let Ok(file) = File::open(root_path.join("index.html")) else {
            dbg!(request.url, request.headers);
            return Some(Response::empty(StatusCode(404)));
        };

        return Some(Response::file(file));
        };

    let Ok(file) = File::open(root_path.join(target)) else {
        return Some(Response::empty(StatusCode(404)));
    };

    Some(Response::file(file))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Root node is equivalent of `/`
    // You do not need to construct this as `Service::root()`, constructing any other service node
    // is fine, the name is just ignored.
    // adding a param will allow you to `get_url_value` from a request, where the next url "segment" is the value
    let root = Service::root()
        .add_system(System::single(root))
        .add_param(String::from("file"));

    let app = Application::new("0.0.0.0:80", root)?;

    app.run()?;

    Ok(())
}
