// Import modules
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

enum ConnectionState {
    NewUser,
    User(String),
}

// Use our custom module
mod errors;

// TODO: 
// What is bad here?
// 1. let _ = is rubbish here, handle the errors.

fn create_pair(read_stream: TcpStream) -> errors::Result<(Reader, Writer)> {
    let write_stream = read_stream.try_clone()?;
    let (sender, receiver) = mpsc::channel();
    let reader = Reader::new(read_stream, sender);
    let writer = Writer { stream: write_stream, receiver };

    Ok((reader, writer))
}

struct Reader {
    stream: TcpStream,
    sender: mpsc::Sender<String>,
    buffer: Vec<u8>,
    state: ConnectionState,
}

impl Reader {
    pub fn new(stream: TcpStream, sender: mpsc::Sender<String>) -> Self {
        let buffer = vec![0; 1024];

        Self {
            stream,
            sender,
            buffer,
            state: ConnectionState::NewUser,
        }
    }
}

struct Writer {
    stream: TcpStream,
    receiver: mpsc::Receiver<String>,
}

fn handle_client(reader: Reader, writer: Writer, addr: SocketAddr, connections: Arc<Mutex<Vec<mpsc::Sender<String>>>>) {
    println!("Connection from : {:?}", addr);

    // Add connection to `connections`
    let _ = connections.lock().map(|mut cons| cons.push(reader.sender.clone()));

    // Reader thread
    thread::spawn(move || handle_read(reader, connections));

    // Writer thread
    thread::spawn(move || handle_write(writer));
}

fn handle_read(mut reader: Reader, connections: Arc<Mutex<Vec<mpsc::Sender<String>>>>) {
    loop {
        // Match
        match reader.stream.read(&mut reader.buffer) {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                // `n` is the number of bytes read

                let msg_bytes = &reader.buffer[..n];

                // Create a `String` from the bytes
                // String has to be valid utf8
                match String::from_utf8(msg_bytes.to_vec()) {
                    Ok(mut msg) => { 
                        match reader.state {
                            ConnectionState::NewUser => {
                                msg.pop();
                                reader.state = ConnectionState::User(msg);
                            }
                            ConnectionState::User(ref name) => {
                                let mut cons = connections.lock().expect("failed to aquire lock");
                                let msg = format!("{} > {}", name, msg);
                                cons.iter_mut().for_each(|sender| {
                                    sender.send(msg.clone()).expect("failed to send message");
                                });
                            }
                        }
                    }
                    Err(e) => eprintln!("Utf8 err: {:?}", e),
                }

            }
            Err(e) => {
                eprintln!("Read err: {:?}", e);
                break;
            }
        }
    }

    eprintln!("Connection closed");
}

fn handle_write(mut writer: Writer) -> errors::Result<()> {
    while let Ok(msg) = writer.receiver.recv() {
        writer.stream.write_all(msg.as_bytes())?;
    }

    Ok(())
}

fn server(addr: &str, max_clients: usize) -> errors::Result<()> {
    // Variable bindings
    let listener = TcpListener::bind(addr)?;
    let connections = Arc::new(Mutex::new(Vec::new()));

    // Loops
    loop {
        while let Ok((stream, addr)) = listener.accept() {
            let client_count = connections.lock().unwrap().len();
            eprintln!("cc: {} | max: {}", client_count, max_clients);

            // ... and control flow
            if client_count >= max_clients {
                // We don't want to accept more clients
                continue;
            }

            if let Ok((reader, writer)) = create_pair(stream) {
                handle_client(reader, writer, addr, Arc::clone(&connections));
            }

        }
    }
}


