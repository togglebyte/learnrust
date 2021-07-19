
fn print(s: &str) {
    eprintln!("{}", s);
}

fn main() {

    // When to use String and when to use string slice (&str)
    // * String when you have to modify it
    // * &str when you don't

    // Owned
    let mut my_string: String = String::new(); // Wrapper around Vec<u8>
    my_string.push_str("hello world");
    my_string.push_str(" hello again");

    // Borrow
    let s: &str = "    This is a string literal";
    // let s_slice: &str = &my_string;
    
    print(&my_string);
    print(&my_string);


    let bunny = "1".to_string();
    let string_bytes: Vec<u8>  = bunny.into_bytes();
    eprintln!("{:?}", string_bytes[0] as char);
    // eprintln!("len: {:?}", bunny.len());
}
