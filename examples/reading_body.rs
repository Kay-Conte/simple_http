use simple_http::{Application, Request, Response, Service, StatusCode, System, Command};

fn root(req: &mut Request) -> Command {
    // Don't expect users to always send valid data in a real application
    let body = req.body_to_string().expect("Failed to parse body");

    println!("{}", body);

    Command::Respond(Response::empty(StatusCode(200)))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = Service::root().add_system(System::single(root));

    let app = Application::new("0.0.0.0:22555", root)?;

    let _ = app.run();

    Ok(())
}
