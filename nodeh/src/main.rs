use nodeh::parser;

use html5ever::tendril::TendrilSink;

use std::default::Default;
use std::path::Path;

use html5ever::parse_document;

fn main() {
    let sink = parser::Parser::new();

    parse_document(sink, Default::default())
        .from_utf8()
        .from_file(Path::new("test.html"))
        .unwrap();
}
