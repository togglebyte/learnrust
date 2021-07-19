use std::net::TcpListener;
// ADD: arc and mutex
use std::sync::{Arc, Mutex};

// ADD: Add connection module
mod connections;

struct Config {
    addr: String,
    max_connections: usize,
}

fn start_server(config: Config) {
    let listener = TcpListener::bind(&config.addr).expect("Failed to start server");

    let clients = Vec::with_capacity(config.max_connections);
    // CHANGE: Change clients to hold an Arc<Mutex<T>> 
    // (introduce shadowing)
    let clients = Arc::new(Mutex::new(clients));

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                // CHANGE: lock and check client length.
                // Make sure not to retain the lock longer than necessary.
                let locked_clients = clients.lock().expect("Failed to lock clients");
                if locked_clients.len() >= config.max_connections {
                    // We don't accept new clients so we simply
                    // continue, thus closing the connection
                    continue;
                }
                // Drop the lock.
                drop(locked_clients);

                // CHANGE: Change `clients.push` to `connections:handle_connection`
                connections::handle_connection(stream, Arc::clone(&clients));
                eprintln!("New connection from {}", addr);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
                continue;
            }
        }
    }
}

fn main() {
    let config = Config {
        addr: "127.0.0.1:5000".to_string(),
        max_connections: 4,
    };
    start_server(config);
}
