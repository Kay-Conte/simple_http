use std::sync::atomic::{AtomicBool, Ordering};

use simple_http::{service::{Command, Service, System}, request::Request, response::Response, StatusCode, application::Application};

type Data = AtomicBool;

fn json(_req: &mut Request, ctx: &Data) -> Command<Data> {
    ctx.swap(!ctx.load(Ordering::Relaxed), Ordering::Relaxed);

    Command::Respond(Response::empty(StatusCode(200)))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Root node is equivalent of `/`
    let root = Service::root()
        .fold(|s| s.insert_child(Service::with_system("json", System::single(json))));

    let app = Application::new("0.0.0.0:80", root, AtomicBool::new(false))?;

    let _ = app.run();

    Ok(())
}
