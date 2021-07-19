use std::net::TcpListener;
use std::sync::{Arc, Mutex};

mod connections;
// ADD: error module
mod errors;

use errors::{Error, Result};

struct Config {
    addr: String,
    max_connections: usize,
}

fn start_server(config: Config) -> Result<()> {
    let listener = TcpListener::bind(&config.addr)?;

    let clients = Vec::with_capacity(config.max_connections);
    let clients = Arc::new(Mutex::new(clients));

    let mut next_id = 0;
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                // CHANGE: handle errors rather than using `expect`
                let locked_clients = clients.lock()?;
                if locked_clients.len() >= config.max_connections {
                    // We don't accept new clients so we simply
                    // continue, thus closing the connection
                    continue;
                }
                drop(locked_clients);

                // CHANGE: If we return a MutexErr from here, it means
                //         we can't recover as the mutex is poisoned.
                //         The only thing we can do here is end the application.
                let res = connections::handle_connection(next_id, stream, Arc::clone(&clients));
                if let Err(Error::MutexErr) = res {
                    return Err(Error::MutexErr);
                }
                next_id += 1;
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
    // CHANGE: show the error
    if let Err(e) = start_server(config) {
        e.print();
    }
}
