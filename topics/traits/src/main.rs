use std::io::{stdin, Read, Result, ErrorKind};

use crate::traits::Bob;
use crate::traits::{AlsoDesc, Describer};

mod traits;

struct StoreThree<T> {
    a: T,
    b: T,
    c: T,
}

fn reads_input_parse_int_returns_output_maybe() -> Result<u32> {
    let mut stdin = stdin();

    let mut output = String::new();
    let val = stdin.read_line(&mut output);
    let val: u32 = match output.trim().parse() {
        Ok(val) => val,
        Err(_) => return Err(ErrorKind::InvalidInput.into()),
    };

    Ok(val)
}

fn use_a_result() -> Result<()> {
    let val_a = reads_input_parse_int_returns_output_maybe()?;
    let val_b = reads_input_parse_int_returns_output_maybe()?;
    let val_c = reads_input_parse_int_returns_output_maybe()?;

    eprintln!("{:?}", val_a + val_b + val_c);

    Ok(())
}

fn main() {
    match use_a_result() {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

