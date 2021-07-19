use std::io::stdin;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::{Arc, Mutex};


fn slow_comp() {
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let now = Instant::now();

    let comp_count = 8;

    let mut handles = Vec::new();
    for _ in 0..comp_count {
        let handle = thread::spawn(|| {
            slow_comp();
        });

        handles.push(handle);
    }


    eprintln!("{:?}", now.elapsed());

    for handle in handles {
        handle.join();
    }

}

