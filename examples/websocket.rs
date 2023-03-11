use sha1::{Digest, Sha1};

use rustc_serialize::base64::{CharacterSet::Standard, Config, Newline, ToBase64};

use simple_http::{
    Application, Command, Context, Header, Request, Response, Service, StatusCode, System,
    WebsocketDescriptor,
};

use std::io::Read;

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

fn root(req: &mut Request, _ctx: &Context) -> Command {
    let Some(key) = req.headers().iter().find(|h| h.field.equiv(&"Sec-Websocket-Key")).map(|h| h.value.clone()) else {
        return Command::Respond(Response::empty(StatusCode(400)));
    };

    let response = Response::empty(StatusCode(101))
        .with_header("Upgrade: websocket".parse::<Header>().unwrap())
        .with_header(
            "Connection: Upgrade"
                .parse::<Header>()
                .unwrap(),
        )
        
        .with_header(
            format!("Sec-Websocket-Accept: {}", convert_key(key.as_str()))
                .parse::<Header>()
                .unwrap(),
        );

    Command::Upgrade(WebsocketDescriptor::new(response, (), |ws, _ctx, ()| {
        match ws.next_frame() {
            Some(frame) => {
                println!("{:?}", frame); 
                Some(())
            }
            None => None,
        }
    }))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = Service::root().insert_system(System::single(root));

    let app = Application::new("0.0.0.0:22555", root)?;

    let _ = app.run();

    Ok(())
}
