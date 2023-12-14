use std::env;

mod errors;

mod tcp_client;
mod tcp_server;

fn main() {
    let args: Vec<String> = env::args().collect();

    // server or client
    let role: &str = &args[1];

    // addr:port
    let address: &str = &args[2];

    match role {
        "server" => {
            tcp_server::server(address).expect("");
        }
        "client" => {
            tcp_client::connect(address).expect("");
        }
        _ => {
            panic!("server or client");
        }
    }
}
