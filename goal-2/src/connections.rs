use std::net::TcpStream;
use std::sync::{mpsc, Mutex, Arc};
use std::io::{Read, Write};
use std::thread;

enum State {
    Anon,
    User(String),
}

// Type alias introduced
type Connections<T> = Arc<Mutex<Vec<T>>>;

struct Reader {
    stream: TcpStream,
    state: State,
    buf: Vec<u8>,
    connections: Connections<mpsc::Sender<String>>,
}

impl Reader {
    fn new(stream: TcpStream, connections: Connections<mpsc::Sender<String>>) -> Self {
        Self {
            stream,
            state: State::Anon,
            buf: vec![0u8; 1024],
            connections,
        }
    }
}

struct Writer {
    stream: TcpStream,
    receiver: mpsc::Receiver<String>,
}

impl Writer {
    fn new(stream: TcpStream, receiver: mpsc::Receiver<String>) -> Self {
        Self {
            stream, 
            receiver,
        }
    }
}

pub fn handle_connection(reader: TcpStream, connections: Connections<mpsc::Sender<String>>) {
    let writer = reader.try_clone().expect("Failed to clone socket");

    let (sender, receiver) = mpsc::channel();
    let reader = Reader::new(reader, Arc::clone(&connections));
    let writer = Writer::new(writer, receiver);

    let mut locked_con = connections.lock().expect("Failed to acquire lock");
    locked_con.push(sender);
    drop(locked_con);

    thread::spawn(move || {
        handle_reader(reader);
    });

    thread::spawn(move || {
        handle_writer(writer);
    });
}

fn handle_reader(mut reader: Reader) {
    loop {
        match reader.stream.read(&mut reader.buf) {
            Ok(0) => break,
            Ok(n) => {
                let msg = &reader.buf[..n];
                let msg = std::str::from_utf8(msg).expect("Failed to create valid utf8 string");
                let msg = msg.trim().to_string();
                match reader.state {
                    State::Anon => reader.state = State::User(msg),
                    State::User(ref username) => {
                        let mut locked_con = reader.connections.lock().expect("Failed to acquire lock");
                        for sender in locked_con.as_mut_slice() {
                            sender.send(format!("{} > {}\n", username, msg));
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("failed to read from socket: {:?}", e);
                break
            }
        }
        
    }

    println!("Connection closed (reader)");

    let Reader { state, connections, .. } = reader;

    let username = match state {
        State::Anon => "[anon]".to_string(),
        State::User(name) => name,
    };

    let mut locked_con = connections.lock().expect("Failed to acquire lock");
    for sender in locked_con.as_mut_slice() {
        sender.send(format!("-- {} left --\n", username));
    }
}

fn handle_writer(mut writer: Writer) {
    loop {
        let msg = match writer.receiver.recv() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to receive message: {:?}", e);
                break;
            }
        };

        // Mention the use of `_`
        if let Err(_e) = writer.stream.write_all(msg.as_bytes()) {
            break
        }
    }

    println!("Connection closed (writer)");
}
