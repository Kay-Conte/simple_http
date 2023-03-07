use simple_http::{Service, Application, System, Request, Response, StatusCode};

fn hello_world(_req: &Request) -> Option<Response> {
    // Responding with `None` will act as a middleware System
    // Responding with `Some` will respond to the request object and move on to the next request
    // All systems registered after receiving a `Some` will not be run
    
    Some(Response::empty(StatusCode(200)))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = Service::root()
        .fold(|s| {
            // `localhost/hello_world`
            s.insert_child(Service::with_system("hello_world", System::single(hello_world)))
        });

    // The application will automatically respond to all unrecognized urls with a `StatusCode(404)` not found
    // In this case, the only recognized url is `localhost/hello_world`
    let app = Application::new("0.0.0.0:80", root)?;

    // Initiate main application loop
    let _ = app.run();

    Ok(())
}