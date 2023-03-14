use simple_http::{service::{Command, Service, System}, request::Request, response::Response, StatusCode, application::Application};

type Data = ();

fn root(req: &mut Request, _ctx: &Data) -> Command<Data> {
    // Don't expect users to always send valid data in a real application
    let body = req.body_to_string().expect("Failed to parse body");

    println!("{}", body);

    Command::Respond(Response::empty(StatusCode(200)))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = Service::root().insert_system(System::single(root));

    let app = Application::new("0.0.0.0:22555", root, ())?;

    let _ = app.run();

    Ok(())
}
