use std::net::TcpListener;

struct Config {
    addr: String,
    max_connections: usize,
}

fn start_server(config: Config) {
    let listener = TcpListener::bind(&config.addr).expect("Failed to start server");

    let mut clients = Vec::with_capacity(config.max_connections);

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {

                if clients.len() >= config.max_connections {
                    // We don't accept new clients so we simply
                    // continue, thus closing the connection
                    continue;
                }

                clients.push(stream);
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
        max_connections: 2,
    };
    start_server(config);
}
