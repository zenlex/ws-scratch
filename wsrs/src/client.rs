use tungstenite::{connect, Message};
use url::Url;
pub fn init_client() {
    env_logger::init();

    let (mut socket, response) =
        // connect(Url::parse("ws://zenlex.imperfect.systems/socket").unwrap())
        //     .expect("Can't connect");
        connect(Url::parse("ws://127.0.0.1:3012").unwrap())
            .expect("Can't connect");

    println!("Connected to server");
    println!("Response HTTP status:{}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header)
    }

    loop {
        let mut line = String::new();
        println!("Enter some message text");
        std::io::stdin().read_line(&mut line).unwrap();
        socket.write_message(Message::Text(line).into()).unwrap();

        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
