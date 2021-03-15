use nodeh::parser;

use stdlib;

use func::Lib;

use html5ever::tendril::TendrilSink;

use std::default::Default;
use std::path::Path;

use html5ever::parse_document;

fn main() {
    let sink = parser::Parser::new(load());
    parse_document(sink, Default::default())
        .from_utf8()
        .from_file(Path::new("test.html"))
        .unwrap();
}

fn load() -> Lib {
    let mut functions: Lib = Vec::new();

    functions.append(&mut stdlib::register());

    functions
}
