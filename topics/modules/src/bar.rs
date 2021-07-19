use crate::foo;
use crate::foo::do_it;

pub fn use_foo() {
    foo::do_it();
    do_it();
}

pub fn thing() {
}

