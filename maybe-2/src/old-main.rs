// Imports
use std::io::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};

// -----------------------------------------------------------------------------
//     - 5. Structs -
//     Create a reader and writer struct that represents
//     the connection.
// -----------------------------------------------------------------------------
struct Reader {
    stream: TcpStream,
    sender: mpsc::Sender<Vec<u8>>,
}

struct Writer {
    stream: TcpStream,
    receiver: mpsc::Receiver<Vec<u8>>,
}

// Create a `Reader` and `Writer` pair from a connection.
// We need both reader and writer because they will live on 
// separate threads.
fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    connections: Arc<Mutex<Vec<(SocketAddr, mpsc::Sender<Vec<u8>>)>>>
) {
    let writer = match stream.try_clone() {
        Ok(writer) => writer,
        Err(e) => {
            eprintln!("Failed to create reader / writer pair. Reason: {:?}", e);
            return;
        }
    };

    // -----------------------------------------------------------------------------
    //     - 8. Channels -
    // -----------------------------------------------------------------------------
    let (sender, receiver) = mpsc::channel();

    // Add clients `sender` to the list of connections.
    // This means we can iterate over all the senders and send messages
    // to all the clients.
    //
    // We do this in a block so the `lock` is dropped 
    // at the end of the scope.
    {
        let mut cons = match connections.lock() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to lock the mutex. Reason: {:?}", e);
                return;
            }
        };
        // Push (add) the sender and the address on to the vector
        cons.push((addr, sender.clone()));
    } // Drop the lock

    // create a reader / writer pair and send them them 
    // to their respective threads
    let reader = Reader { stream, sender };
    let writer = Writer { stream: writer, receiver };
}

// -----------------------------------------------------------------------------
//     - 2. Functions -
// -----------------------------------------------------------------------------
fn serve(addr: &str, max_connections: usize) {
    // -----------------------------------------------------------------------------
    //     - 3. Match -
    // -----------------------------------------------------------------------------
    // Assign listener through a match expression.
    //
    // Create a tcp listener.
    // If this fails we print the error message to `stderr`,
    // and return from the function.
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to create listner. Reason: {:?}", e);
            return; // exit the function early
        }
    };

    let mut total_connections_received = 0;
    let connections = Arc::new(Mutex::new(Vec::new()));

    // -----------------------------------------------------------------------------
    //     - 4. Loops and control flow -
    // -----------------------------------------------------------------------------
    // Note that `(con, addr)` is a tuple.
    loop {
        // -----------------------------------------------------------------------------
        //     - 6. Mutability -
        // -----------------------------------------------------------------------------
        total_connections_received += 1;
        eprintln!("Number of connections received: {}", total_connections_received);

        match listener.accept() {
            Ok((stream, addr)) => handle_connection(stream, addr, Arc::clone(&connections)),
            Err(e) => {
                eprintln!("Failed to handle connection. Reason: {:?}", e);
                continue; // continue the loop, so we can accept a new connection
            }
        }
    }
}

fn main() {
    // -----------------------------------------------------------------------------
    //     - 1. Variable bindings -
    // -----------------------------------------------------------------------------
    let addr = "127.0.0.1:5000";
    let max_connections = 10;

    serve(addr, max_connections);
}
