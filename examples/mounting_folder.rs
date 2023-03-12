use std::fs::File;

use simple_http::{Application, Command, Param, Request, Response, Service, StatusCode, System};

type Data = ();

// This example should be run from the project root directory using `cargo run --example hello_world`

// A system that returns `None` does not produce a response and simply acts as middleware. Services
// may have multiple systems and they will always be executed in order. A system that returns
// `Some(...)` will stop the task and produce a response to the request.

fn root(req: &mut Request, _ctx: &Data) -> Command {
    let mut path = std::env::current_dir()
        .expect("Failed to get working directory")
        .join("examples/html");

    // This value should always exist. Param values are always inserted
    let segments = req.get_url_value("file").expect("Param value not found");

    for target in segments {
        path = path.join(target);
    }

    if path.is_dir() {
        path = path.join("index.html");
    }

    let Ok(file) = File::open(path) else {
        return Command::Respond(Response::empty(StatusCode(404)));
    };

    Command::Respond(Response::file(file))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Root node is equivalent of `/`
    // You do not need to construct this as `Service::root()`, constructing any other service node
    // is fine, the name is just ignored.
    // adding a param will allow you to `get_url_value` from a request, where the next url "segment" is the value
    let root = Service::root()
        .insert_system(System::single(root))
        .insert_param(Param::CollectAll("file".to_string()));

    let app = Application::new("0.0.0.0:22555", root, ())?;

    app.run()?;

    Ok(())
}
