use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message,
};

pub fn start_server() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Receieved a new handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The requests's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() || msg.is_text() {
                    println!("Server recieved message: {}", msg);

                    let mut return_msg = String::from("From server: ");
                    return_msg.push_str(&msg.into_text().unwrap());

                    websocket.write_message(Message::Text(return_msg)).unwrap();
                }
            }
        });
    }
}
