use core::{Handle, Node};
use parser::Parser;
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
                // TODO consider possibly not passing children if not required by func
                (e.func.handler)(
                    e.matched_attributes.clone(),
                    handles_to_nodes(e.children.clone(), tree.clone()),
                )
                .unwrap();
            }

            if !(e.func.accepted_body.accept_text
                || e.func.accepted_body.accepted_funcs.len() > 0
                || e.func.accepted_body.accepted_elements.len() > 0)
            {
                for child in e.children.clone() {
                    run(tree.clone(), child)
                }
            }
        }
        _ => {}
    }
}

fn handles_to_nodes(handles: Vec<Handle>, tree: Parser) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    for handle in handles {
        nodes.push(
            tree.nodes
                .get(&handle)
                .expect(format!("No element {} found", handle).as_str())
                .clone(),
        )
    }

    nodes
}
