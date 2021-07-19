use serde_json::Value;

pub mod bar;
pub mod foo;

mod parent {
    pub fn be_parenty() {
        child::disable_cake();
    }

    mod child {
        fn disable_cake() {
            super::be_parenty();
        }
    }
}


fn main() {
    parent::be_parenty();
}

