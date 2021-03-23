use crate::parser::{Handle, Node, Parser};
use Node::IsElement;

pub fn execute(tree: Parser) {
    run(tree, 0)
}

fn run(tree: Parser, id: Handle) {
    let node = tree
        .nodes
        .get(&id)
        .expect(format!("No element {} found", id).as_str());

    match node {
        IsElement(e) => {
            if e.is_func {
                (e.func.handler)(e.matched_attributes.clone()).unwrap();
            }

            for child in e.children.clone() {
                run(tree.clone(), child)
            }
        }
        _ => {}
    }
}
