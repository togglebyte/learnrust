use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// ADD: custom result
use crate::errors::Result;

// ADD: message enum
pub enum Message {
    Quit(usize, Option<String>),
    Data {
        sender: usize,
        username: String,
        text: String,
    },
}

enum State {
    Anon,
    User(String),
}

type Connections<T> = Arc<Mutex<Vec<T>>>;

// CHANGE: add id
struct Reader {
    stream: TcpStream,
    state: State,
    buf: Vec<u8>,
    connections: Connections<(usize, mpsc::Sender<Message>)>,
    id: usize,
}

impl Reader {
    // CHANGE: hold id as well as `Sender<Message>`
    fn new(id: usize, stream: TcpStream, connections: Connections<(usize, mpsc::Sender<Message>)>) -> Self {
        Self {
            stream,
            state: State::Anon,
            buf: vec![0u8; 1024],
            connections,
            id,
        }
    }
}

// CHANGE: add id
struct Writer {
    stream: TcpStream,
    receiver: mpsc::Receiver<Message>,
    id: usize,
}

impl Writer {
    fn new(id: usize, stream: TcpStream, receiver: mpsc::Receiver<Message>) -> Self {
        Self {
            stream,
            receiver,
            id,
        }
    }
}

// CHANGE: add the id and return an error::Result
pub fn handle_connection(
    id: usize,
    reader: TcpStream,
    // CHANGE: change connections to include the id
    connections: Connections<(usize, mpsc::Sender<Message>)>,
) -> Result<()> {
    let writer = reader.try_clone()?;

    let (sender, receiver) = mpsc::channel();
    let reader = Reader::new(id, reader, Arc::clone(&connections));
    let writer = Writer::new(id, writer, receiver);

    let mut locked_con = connections.lock()?;
    locked_con.push((id, sender));
    drop(locked_con);

    thread::spawn(move || {
        // CHANGE: `let _ = ` disposes of the result
        //         as we can't handle the result inside the thread.
        if let Err(e) = handle_reader(reader) {
            e.print();
        }
        println!("Connection closed (reader)");
    });
    thread::spawn(move || {
        if let Err(e) = handle_writer(writer) {
            e.print();
        }
        println!("Connection closed (writer)");
    });

    Ok(())
}

// CHANGE: return a `Result<()>`
fn handle_reader(mut reader: Reader) -> Result<()> {
    loop {
        // CHANGE: we use `?` op here as we deal with the
        //         error message inside the thread running
        //         this function.
        let bc = reader.stream.read(&mut reader.buf)?;

        if bc == 0 {
            break;
        }

        // CHANGE: do this outside of the previous match statement
        //         as we are now dealing with the errors elsewhere
        let msg = &reader.buf[..bc];
        let msg = std::str::from_utf8(msg)?;
        let msg = msg.trim().to_string();

        match reader.state {
            State::Anon => reader.state = State::User(msg),
            State::User(ref username) => {
                // CHANGE: `?` op
                let mut locked_con = reader.connections.lock()?;
                for (_, sender) in locked_con.as_mut_slice() {
                    // UPDATE: change to send `Message` rather than `String`
                    sender.send(Message::Data {
                        username: username.clone(),
                        sender: reader.id,
                        text: msg.clone(),
                    })?;
                }
            }
        }
    }

    let Reader {
        state,
        connections,
        id,
        ..
    } = reader;

    // CHANGE: Username is now an option on account of anon users
    let username = match state {
        State::Anon => None,
        State::User(name) => Some(name),
    };

    // CHANGE: `?` op
    let mut locked_con = connections.lock()?;

    for (_, sender) in locked_con.as_mut_slice() {
        // CHANGE: send quit message
        // CHANGE: `?` as failing to send is unrecoverable
        sender.send(Message::Quit(id, username.clone()))?;
    }
    // CHANGE: include the id 
    locked_con.retain(|(connection_id, _)| *connection_id != id);

    Ok(())
}

fn handle_writer(mut writer: Writer) -> Result<()> {
    loop {
        let msg = match writer.receiver.recv() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to receive message: {:?}", e);
                break;
            }
        };

        // CHANGE: use a message enum instead
        //         so we can unblock the writer
        match msg {
            Message::Quit(id, _) if id == writer.id => break,
            Message::Quit(_, Some(username)) => {
                let message = format!("[{} left]\n", username);
                if let Err(_e) = writer.stream.write_all(message.as_bytes()) {
                    break;
                }
            }
            Message::Quit(_, _) => {}
            Message::Data {
                sender,
                username,
                text,
            } => {
                let sender = if sender == writer.id {
                    "[you]"
                } else {
                    username.as_str()
                };

                let message = format!("{} > {}\n", sender, text);
                if let Err(_e) = writer.stream.write_all(message.as_bytes()) {
                    break;
                }
            }
        }
    }

    Ok(())
}
