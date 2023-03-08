use std::str;

use simple_http::{Application, Request, Response, Service, StatusCode, System};

fn root(req: &Request) -> Option<Response> {
    // Don't expect users to always send valid data in a real application
    let body = str::from_utf8(&req.body).expect("Failed to parse body from string");

    println!("{body}");

    Some(Response::empty(StatusCode(200)))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = Service::root().add_system(System::single(root));

    let app = Application::new("0.0.0.0:80", root)?;

    let _ = app.run();

    Ok(())
}
