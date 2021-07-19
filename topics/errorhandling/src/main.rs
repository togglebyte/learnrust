use std::io::{Read, Error as IoErr};
use std::str::{from_utf8, Utf8Error};
use std::net::TcpStream;

// -----------------------------------------------------------------------------
//     - Use anyhow::Result -
//     If you uncomment this line: // use anyhow::Result;
//     and remove the type alias the code will still compile.
// -----------------------------------------------------------------------------
// use anyhow::Result;

type Result<T> = std::result::Result<T, OurError>;

// -----------------------------------------------------------------------------
//     - Declare our error type -
//     Since we implement `From` for both Io errors and Utf8 errors
//     We can use the question mark operator which will "magically" call "into"
//     for us.
//
//     I.e: `e: OurErr = error.into()`
// -----------------------------------------------------------------------------
#[derive(Debug)]
enum OurError {
    Io(IoErr),
    Utf8(Utf8Error),
}

impl From<IoErr> for OurError {
    fn from(e: IoErr) -> Self {
        Self::Io(e)
    }
}

impl From<Utf8Error> for OurError {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

// -----------------------------------------------------------------------------
//     - Fancy network program -
// -----------------------------------------------------------------------------
fn run() -> Result<()> {

    let mut buf = vec![0u8; 8192];

    let mut stream = TcpStream::connect("127.0.0.1:5000")?;
    loop {
        let bc = stream.read(&mut buf)?;

        if bc == 0 {
            eprintln!("Bye bye");
            return Ok(());
        }

        let msg = match from_utf8(&buf[..bc]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Invalid utf8: {:?}", e);
                continue;
            }
        };
    }

    eprintln!("all is well");
    Ok(())
}

// -----------------------------------------------------------------------------
//     - Main -
//     We pass all error messages down to `main` and print them there
// -----------------------------------------------------------------------------
fn main() {
    if let Err(e) = run() {
        eprintln!("Fail: {:?}", e);
    }
}
