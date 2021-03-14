use html5ever::tendril::TendrilSink;
use nodeh::parser;

use std::collections::HashMap;
use std::default::Default;
use std::path::Path;

use html5ever::parse_document;

fn main() {
    let sink = parser::Sink {
        next_id: 1,
        names: HashMap::new(),
        line: 0,
    };

    parse_document(sink, Default::default())
        .from_utf8()
        .from_file(Path::new("test.html"))
        .unwrap();
}
