use sha1::{Digest, Sha1};

use rustc_serialize::base64::{CharacterSet::Standard, Config, Newline, ToBase64};

use simple_http::{Application, Command, Header, Request, Response, Service, StatusCode, System};

use std::io::Read;

type Data = ();

fn convert_key(input: &str) -> String {
    let mut input = input.to_string().into_bytes();
    let mut bytes = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
        .to_string()
        .into_bytes();
    input.append(&mut bytes);

    let mut sha = Sha1::new();
    sha.update(&input);

    sha.finalize()
        .bytes()
        .map(|e| e.unwrap())
        .collect::<Vec<u8>>()
        .to_base64(Config {
            char_set: Standard,
            pad: true,
            line_length: None,
            newline: Newline::LF,
        })
}

fn root(req: &mut Request, _ctx: &Data) -> Command {
    let Some(key) = req.headers().iter().find(|h| h.field.equiv(&"Sec-Websocket-Key")).map(|h| h.value.clone()) else {
        return Command::Respond(Response::empty(StatusCode(400)));
    };

    let mut response = Response::empty(StatusCode(101))
        .with_header("Upgrade", "websocket")
        .unwrap()
        .with_header("Connection", "Upgrade")
        .unwrap()
        .with_header("Sec-Websocket-Accept", &convert_key(key.as_str()))
        .unwrap();

    Command::Respond(response)
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = Service::root().insert_system(System::single(root));

    let app = Application::new("0.0.0.0:22555", root, ())?;

    let _ = app.run();

    Ok(())
}
