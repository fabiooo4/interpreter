// Suppress all warnings from generated code
#![allow(warnings)]

pub mod intexprlexer {
    include!(concat!(env!("OUT_DIR"), "/intexprlexer.rs"));
}

pub mod intexprparser {
    include!(concat!(env!("OUT_DIR"), "/intexprparser.rs"));
}

pub mod intexprlistener {
    include!(concat!(env!("OUT_DIR"), "/intexprlistener.rs"));
}
