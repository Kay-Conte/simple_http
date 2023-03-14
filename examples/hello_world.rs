use simple_http::{service::{Command, Service, System}, request::Request, response::Response, StatusCode, application::Application};

type Data = ();

fn hello_world(_req: &mut Request, _ctx: &Data) -> Command<Data> {
    // Responding with `None` will act as a middleware System
    // Responding with `Some` will respond to the request object and move on to the next request
    // All systems registered after receiving a `Some` will not be run

    Command::Respond(Response::empty(StatusCode(200)))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = Service::root().fold(|s| {
        // `localhost/hello_world`
        s.insert_child(Service::with_system(
            "hello_world",
            System::single(hello_world),
        ))
    });

    // The application will automatically respond to all unrecognized urls with a `StatusCode(404)` not found
    // In this case, the only recognized url is `localhost/hello_world`
    let app = Application::new("0.0.0.0:22555", root, ())?;

    // Initiate main application loop
    let _ = app.run();

    Ok(())
}
