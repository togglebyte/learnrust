use std::thread;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Mutex, Arc};

enum State {
    Anon,
    User(String),
}

// -----------------------------------------------------------------------------
//     - Connection trait -
// -----------------------------------------------------------------------------
trait Connection {
    fn from_stream(stream: TcpStream) -> Self;

    fn send(&mut self, data: &[u8]);
    fn receive(&mut self);
}

// -----------------------------------------------------------------------------
//     - Clients -
// -----------------------------------------------------------------------------
struct Client {
    stream: TcpStream,
    state: State,
    buf: Vec<u8>,
}

impl Connection for Client {
    fn from_stream(stream: TcpStream) -> Self {
        Self {
            stream,
            state: State::Anon,
            buf: vec![0u8; 128],
        }
    }

    fn send(&mut self, data: &[u8]) {
        self.stream.write(data);
    }

    fn receive(&mut self) {
        match self.stream.read(&mut self.buf) {
            Ok(n) => {
                let data = self.buf[..n].to_vec();
                let mut s = String::from_utf8(data).expect("Not valid UTF8");
                match self.state {
                    State::Anon => {
                        eprintln!("Username set to: {}", s);
                        s.pop();
                        self.state = State::User(s);
                    }
                    State::User(ref username) => println!("{} > {}", username, s),
                }
            }
            Err(e) => eprintln!("Failed to read from the socket: {:?}", e),
        }
    }
}

struct DebugClient(Client);

impl Connection for DebugClient {
    fn from_stream(stream: TcpStream) -> Self {
        Self(Client::from_stream(stream))
    }

    fn send(&mut self, data: &[u8]) {
        self.0.send(data);
        println!("data sent");
    }

    fn receive(&mut self) {
        println!("Connection received some data");
        self.0.receive();
    }
}


// -----------------------------------------------------------------------------
//     - Config -
// -----------------------------------------------------------------------------
struct Config {
    addr: String,
    max_connections: usize,
}

fn start_server<T: Connection + Send + 'static>(config: Config) {
    let listener = TcpListener::bind(&config.addr).expect("Failed to start server");

    let mut connections = Vec::with_capacity(config.max_connections);

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {

                if connections.len() >= config.max_connections {
                    // We don't accept new connections so we simply
                    // continue, thus closing the connection
                    continue;
                }

                let mut connection = T::from_stream(stream);
                println!("New connection from: {:?}", addr);
                connection.send(b"Welcome\n");
                connections.push(addr);

                // Read thread
                thread::spawn(move || {
                    loop {
                        connection.receive();
                    }

                    println!("Connection closed");
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
                continue;
            }
        }
    }
}

fn main() {
    // TODO: 
    // * handle closing the connection
    // * add channels
    // * Mutex and Arc
    let config = Config {
        addr: "127.0.0.1:5000".to_string(),
        max_connections: 2,
    };
    start_server::<DebugClient>(config);
}
