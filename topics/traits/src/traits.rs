// -----------------------------------------------------------------------------
//     - Traits -
// -----------------------------------------------------------------------------
pub trait Describer {
    fn describe(&self);
}

pub trait AlsoDesc {
    fn describe(&self);

    fn greet(&self) {
    }
}

// -----------------------------------------------------------------------------
//     - Alice -
// -----------------------------------------------------------------------------
pub struct Alice {
}

impl Alice {
    fn new() -> Self {
        Self {
        }
    }
}


// -----------------------------------------------------------------------------
//     - Bob -
// -----------------------------------------------------------------------------
pub struct Bob {
}

impl Bob {
    // Associated function
    // for creating a new Bob
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn inst_of_bob(&self) {
        println!("hi");
    }
}

impl Describer for Bob {
    fn describe(&self) {
        eprintln!("this is a Bob");
    }
}

impl AlsoDesc for Bob {
    fn describe(&self) {
        eprintln!("this is also a Bob");
    }

    fn greet(&self) {
        eprintln!("ello");
    }
}

impl Describer for Alice {
    fn describe(&self) {
        eprintln!("this is an Alice");
    }
}

impl Describer for String {
    fn describe(&self) {
        eprintln!("Look ma, I am a string");
    }
}

