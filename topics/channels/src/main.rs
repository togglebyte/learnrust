use std::time::Duration;
use std::thread;
use std::io::{stdout, stdin};
use std::sync::mpsc::{channel, Sender, Receiver};

fn output(receiver: Receiver<String>, give_back_string: Sender<String>) {
    loop {
        match receiver.recv() {
            Ok(value) => {
                eprintln!("{}", value);
                give_back_string.send(value);
            }
            Err(e) => {
                eprintln!("Channel breakdown: {:?}", e);
                break
            }
        }
    }
}

fn input(sender: Sender<String>, gimme_my_string_back: Receiver<String>) {
    let mut stdin = stdin();

    let mut buf = String::new();
    loop {
        stdin.read_line(&mut buf);
        sender.send(buf);
        buf = match gimme_my_string_back.recv() {
            Ok(buf) => buf,
            Err(e) => break,
        }
    }
}

fn tick(sender: Sender<String>) {
    loop {
        thread::sleep(Duration::from_secs(1));
        sender.send("-tick-".to_string());
    }
}

fn main() {
    let (sender, receiver) = channel();
    let (give_back_sender, give_back_receiver) = channel();

    let sender_clone = sender.clone();
    thread::spawn(move || tick(sender_clone));
    thread::spawn(move || input(sender, give_back_receiver));
    let output_handle = thread::spawn(move || output(receiver, give_back_sender));

    output_handle.join();
    eprintln!("Finish");
}
