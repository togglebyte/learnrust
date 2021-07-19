use std::mem::size_of_val;

fn use_slice(bytes: &[u8]) {
    eprintln!("{:#?}| len: {}", bytes, bytes.len());
}

fn accept_conections() {
    let mut connections = Vec::new();
    let mut next_connection_id: u32 = 0;

    // Loop and accept new connections
    // let connection = server.accept();
    connections.push(next_connection_id);
    next_connection_id += 1;
}

fn main() {
    let mut vec: Vec<u8> = vec![1, 2, 3, 4];
    // let ref_vec: &Vec<u8> = &vec;
    let left: &mut [u8] = &mut vec[0..1];
    let right: &[u8] = &vec[1..];


    let mut slice = &mut vec[..];

    let (left, right) = slice.split_at_mut(1);


    eprintln!("{:?}", left);
    eprintln!("{:?}", right);

    // eprintln!("{:?}", vec);
    // eprintln!("{:?}", slice);

    // let byte: u8 = 1;
    // let ref_byte: &u8 = &byte;
}

// [1, 2, 3, 4]
//    [2, 3]
