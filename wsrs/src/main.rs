mod client;
mod server;

fn main() {
    let mut cmd = String::new();
    println!("Enter command - 'server' to start server, 'client' to start client:");
    std::io::stdin().read_line(&mut cmd).unwrap();
    match cmd.trim() {
        "client" => crate::client::init_client(),
        "server" => crate::server::start_server(),
        _ => panic!("Invalid command: {}", cmd),
    }
}
