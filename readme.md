# Simple Http

#### The problem
The rust http server framework genre seems to be filled with a large overuse of macros to produce framework structures that are essentially a sub-language of rust.

#### The solution
Simple Http is focused on simplicity while still maintaining speed. This library avoids complex, unclear, or otherwise unnecessary macros preferring instead to write (at times) more verbose yet clear; clean code.

## Checklist

Order of tasks not necessarily in order of completion

- [x] Basic routing

- [x] URL parameters

- [x] Multithreading responses

- [ ] Application wide resources/context

- [ ] Method routing? (This may be done through middleware)

- [ ] Websockets

- [ ] Ssl/Https

- [ ] Unknown

## Example

Run examples using `cargo run --example example_name`

A basic hello world program using simple-http:

```rust
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
            s.insert_child(Service::with_system("hello_world", System::single(hello_world)))
        });

    // The application will automatically respond to all unrecognized urls with a `StatusCode(404)` not found
    // In this case, the only recognized url is `localhost/hello_world`
    let app = Application::new("0.0.0.0:80", root)?;

    // Initiate main application loop
    let _ = app.run();

    Ok(())
}
```

## Documentation
Currently not on crates until 0.1.0 release

generate docs using `cargo doc --open`

## Licensing
See LICENSE-MIT
