mod client;
mod minnow;
mod server;

fn main() {
    let mut cmd = String::new();
    println!("Enter command - 'server' to start server, 'client' to start client, 'minnow' to play game:");
    std::io::stdin().read_line(&mut cmd).unwrap();
    match cmd.trim() {
        "client" => crate::client::init_client(),
        "server" => crate::server::start_server(),
        "minnow" => crate::minnow::play_game(),
        _ => panic!("Invalid command: {}", cmd),
    }
}
