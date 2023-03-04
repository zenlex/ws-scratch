use tungstenite::{connect, Message};
use url::Url;

pub fn play_game() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("ws://zenlex.imperfect.systems/socket").unwrap())
            .expect("Can't connect");

    println!("Connected to server");
    println!("Response HTTP status:{}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header)
    }

    let mut grid_size: i32 = 0;
    let mut turn_count: u64 = 0;
    loop {
        // read stream
        let msg = match socket.read_message() {
            Ok(msg) => msg,
            Err(e) => {
                println!("Message read error: {}", e);
                break;
            }
        };
        println!("Receieved:{}", msg);

        let response = calculate_response(
            msg.into_text().expect("invalid string").as_str(),
            &mut grid_size,
            &mut turn_count,
        );

        if response == "end" {
            let _ = socket.close(None);
            // ? - not sure I'm handling this connection closing correctly - am I supposed to continue to flush?
            break;
        }

        // write message to stream
        if response != "" {
            println!("Sending: {}", response);
            match socket.write_message(Message::Text(response)) {
                Ok(val) => val,
                Err(e) => println!("Write Error: {}", e),
            };
        }
    }
}

fn calculate_response(msg: &str, gsize: &mut i32, turn_count: &mut u64) -> String {
    let lc_string = msg.to_ascii_lowercase();
    let tokens = lc_string.split([' ', '\n', ',']).collect::<Vec<&str>>();

    match tokens[0] {
        "hello" => "hello".to_string(),
        "start" => "start".to_string(),
        "size" => {
            *gsize = tokens[1].parse::<i32>().expect("could not parse grid size");
            format!("size set to {}", gsize)
        }
        "run" => "RUNNING!".to_string(),
        "game" => "end".to_string(),
        "shark" => {
            if *turn_count == std::u64::MAX {
                "stop".to_string()
            } else {
                if *turn_count % 10 == 0 {
                    println!(
                        "******************************Turn: {}********************************",
                        turn_count
                    )
                }
                *turn_count += 1;
                calculate_direction(tokens, &gsize)
            }
        }
        "" => "".to_string(),
        _ => panic!("unexpected msg:{:?}", msg),
    }
}

fn calculate_direction(tokens: Vec<&str>, gsize: &i32) -> String {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let shark = Point {
        x: tokens[1].parse().expect("could not parse coordinate"),
        y: tokens[2].parse().expect("could not parse coordinate"),
    };
    let minnow = Point {
        x: tokens[4].parse().expect("could not parse coordinate"),
        y: tokens[5].parse().expect("could not parse coordinate"),
    };

    if minnow.x < 0 || minnow.y < 0 || minnow.x > gsize - 1 || minnow.y > gsize - 1 {
        println!("Minnow out of bounds");
        dbg!(minnow, shark, gsize);
        return "end".to_string();
    }

    let dx = minnow.x - shark.x;
    let dy = minnow.y - shark.y;

    if dx.abs() > dy.abs() {
        // try move in y and turn if at edge
        if minnow.y == 0 || minnow.y == gsize - 1 {
            if dx < 0 && minnow.x > 0 {
                return "left".to_string();
            } else {
                return "right".to_string();
            }
        } else if dy > 0 {
            return "down".to_string();
        } else {
            return "up".to_string();
        }
    } else {
        // try move in x and turn if at edge
        if minnow.x == 0 || minnow.x == gsize - 1 {
            if dy < 0 && minnow.y > 0 {
                return "up".to_string();
            } else {
                return "down".to_string();
            }
        } else if dx < 0 && minnow.x > 0 {
            return "left".to_string();
        } else {
            return "right".to_string();
        }
    }
}
