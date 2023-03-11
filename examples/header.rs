use simple_http::{Application, Command, Context, Request, Response, Service, StatusCode, System};

// This example should be run from the project root directory using `cargo run --example hello_world`

// A system that returns `None` does not produce a response and simply acts as middleware. Services
// may have multiple systems and they will always be executed in order. A system that returns
// `Some(...)` will stop the task and produce a response to the request.

fn json(_req: &mut Request, _ctx: &Context) -> Command {
    let content_type = simple_http::Header::from_bytes(
        &b"Content-Type"[..],
        &b"application/json; charset=UTF-8"[..],
    )
    .unwrap();

    let data = b"{\"data\": \"value\"}";

    Command::Respond(Response::new(
        StatusCode::from(200),
        vec![content_type],
        &data[..],
        Some(data.len()),
        None,
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Root node is equivalent of `/`
    let root = Service::root()
        .fold(|s| s.insert_child(Service::with_system("json", System::single(json))));

    let app = Application::new("0.0.0.0:80", root)?;

    let _ = app.run();

    Ok(())
}
