// Suppress all warnings from generated code
#![allow(warnings)]

pub mod implexer {
    include!(concat!(env!("OUT_DIR"), "/implexer.rs"));
}

pub mod impparser {
    include!(concat!(env!("OUT_DIR"), "/impparser.rs"));
}

pub mod implistener {
    include!(concat!(env!("OUT_DIR"), "/implistener.rs"));
}

pub mod impvisitor {
    include!(concat!(env!("OUT_DIR"), "/impvisitor.rs"));
}
