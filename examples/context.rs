use std::sync::atomic::{AtomicBool, Ordering};

use simple_http::{Application, Command, Context, Request, Response, Service, StatusCode, System};

fn json(_req: &mut Request, ctx: &Context) -> Command {
    let a_bool = ctx
        .data
        .get::<AtomicBool>()
        .expect("No AtomicBool found in type map");

    a_bool.swap(!a_bool.load(Ordering::Relaxed), Ordering::Relaxed);

    Command::Respond(Response::empty(StatusCode(200)))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Root node is equivalent of `/`
    let root = Service::root()
        .fold(|s| s.insert_child(Service::with_system("json", System::single(json))));

    let mut context = Context::default();
    context.data.insert(AtomicBool::new(false));

    let app = Application::with_context("0.0.0.0:80", root, context)?;

    let _ = app.run();

    Ok(())
}
