use std::borrow::Cow;
use std::collections::HashMap;
use std::process::exit;

use html5ever::tendril::*;
use html5ever::tree_builder::{
    AppendNode, AppendText, ElementFlags, NodeOrText, QuirksMode, TreeSink,
};
use html5ever::{local_name, namespace_url, ns, Attribute, ExpandedName, QualName};

use self::Node::{IsElement, IsText};

use func::{empty_func, Func, Lib, Argument};

pub type Handle = usize;

#[derive(Clone)]
pub enum Node {
    IsElement(Element),
    IsText(Text),
}

#[derive(Clone)]
pub struct Text {
    value: String,
}

#[derive(Clone)]
pub struct Element {
    pub element_name: String,
    pub attributes: HashMap<String, String>,
    pub is_func: bool,
    pub func: Func,
    pub matched_attributes: HashMap<String, Argument>,
    pub children: Vec<Handle>,
    qual_name: QualName,
}

#[derive(Clone)]
pub struct Parser {
    next_id: Handle,
    line: u64,
    pub nodes: HashMap<Handle, Node>,
    lib: Lib,
}

impl Parser {
    fn get_id(&mut self) -> Handle {
        let id = self.next_id;
        self.next_id += 2;
        id
    }

    pub fn new(l: Lib) -> Parser {
        let mut new_nodes: HashMap<Handle, Node> = HashMap::new();
        new_nodes.insert(
            0,
            IsElement(Element::new()),
        );

        Parser {
            next_id: 1,
            line: 0,
            nodes: new_nodes,
            lib: l,
        }
    }

    fn get_node(&self, id: &Handle) -> &Node {
        self.nodes
            .get(id)
            .expect(format!("No element {} found", id).as_str())
    }

    fn revise_node(&mut self, node: Node, id: Handle) {
        let mut revised_nodes = self.nodes.clone();
        revised_nodes.insert(id, node);

        self.nodes = revised_nodes;
    }

    fn add_child(&mut self, child: Handle, parent: &Handle) {
        let mut revised_parent = get_element(self.get_node(parent));
        revised_parent.children.push(child);

        self.revise_node(IsElement(revised_parent), *parent)
    }

    fn match_function(&mut self, element: &mut Element) {
        match self.lib.get(&element.element_name) {
            Some(f) => {
                println!("Matched function {} on line {}", element.element_name, self.line);
                element.is_func = true;
                element.func = f.clone();
                self.check_args(element, f.clone())
            },
            None => {}
        }
    }
    
    fn check_args(&self, element: &mut Element, func: Func) {
        let mut e = element.clone();

        'arg: for (arg, spec) in func.args.into_iter() {
            for (name, value) in e.attributes.clone().into_iter() {
                if name == arg {
                    element.matched_attributes.insert(arg, Argument {
                        value: value,
                        ..spec
                    });
                    e.attributes.remove(&name);
                    continue 'arg
                }
            }

            eprintln!("Missing argument '{}' for function {}", arg, element.element_name);
            exit(1);
        }

        if e.attributes.len() > 0 {
            for (name, _) in e.attributes.clone().into_iter() {
                eprintln!("No argument '{}' in function {}", name, element.element_name)
            }

            exit(1);
        }
    }
}

impl Element {
    pub fn new() -> Element {
        Element {
            element_name: String::new(),
            attributes: HashMap::new(),
            is_func: false,
            func: empty_func(),
            matched_attributes: HashMap::new(),
            children: Vec::new(),
            qual_name: QualName::new(None, ns!(html), local_name!("")),
        }
    }
}

impl TreeSink for Parser {
    type Handle = usize;
    type Output = Self;

    fn finish(self) -> Self {
        println!("Finished Parsing!");
        self
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        match msg {
            Cow::Borrowed("Bad DOCTYPE") => {}
            _ => {
                eprintln!("Error Parsing on line {}", self.line);
                exit(1)
            }
        }
    }

    fn get_document(&mut self) -> Handle {
        0
    }

    fn elem_name(&self, target: &Handle) -> ExpandedName {
        let name = self.get_node(target);
        match name {
            IsElement(e) => e.qual_name.expanded(),
            _ => panic!("Not an element"),
        }
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>, _: ElementFlags) -> Handle {
        let id = self.get_id();
        let mut element = Element {
            element_name: name.local.to_string(),
            attributes: get_attributes(attrs),
            qual_name: name,
            ..Element::new()
        };
        self.match_function(&mut element);
        self.nodes.insert(id, IsElement(element));
        id
    }

    fn create_comment(&mut self, _: StrTendril) -> Handle {
        unimplemented!()
    }

    fn create_pi(&mut self, _: StrTendril, _: StrTendril) -> Handle {
        unimplemented!()
    }

    fn append(&mut self, parent: &Handle, child: NodeOrText<Handle>) {
        match child {
            AppendNode(n) => {
                self.add_child(n, parent);
            }
            AppendText(t) => {
                let id = self.get_id();
                let text = Text {
                    value: escape_default(&t),
                };

                self.nodes.insert(id, IsText(text));
                self.add_child(id, parent);
            }
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        _element: &Handle,
        _prev_element: &Handle,
        _child: NodeOrText<Handle>,
    ) {
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        _public_id: StrTendril,
        _system_id: StrTendril,
    ) {
        if name.to_string() != "node.html" {
            eprintln!(
                "DOCTYPE {} is invalid, node.html is required",
                name.to_string()
            );
            exit(1);
        }
    }

    fn get_template_contents(&mut self, _target: &Handle) -> Handle {
        unimplemented!()
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, _mode: QuirksMode) {}

    fn append_before_sibling(&mut self, _sibling: &Handle, _new_node: NodeOrText<Handle>) {}

    fn add_attrs_if_missing(&mut self, target: &Handle, attrs: Vec<Attribute>) {
        let mut revised_attributes = get_element(self.get_node(target)).attributes;

        for attr in attrs {
            if !revised_attributes.contains_key(&attr.name.local.to_string()) {
                revised_attributes.insert(attr.name.local.to_string(), attr.value.to_string());
            }
        }

        let revised_element = Element {
            attributes: revised_attributes,
            ..get_element(self.get_node(target))
        };

        self.revise_node(IsElement(revised_element), *target);
    }

    fn remove_from_parent(&mut self, _target: &Handle) {}

    fn reparent_children(&mut self, _node: &Handle, _new_parent: &Handle) {}

    fn set_current_line(&mut self, line_number: u64) {
        self.line = line_number;
    }
}

fn get_attributes(attrs: Vec<Attribute>) -> HashMap<String, String> {
    let mut attributes: HashMap<String, String> = HashMap::new();

    for attr in attrs {
        attributes.insert(attr.name.local.to_string(), attr.value.to_string());
    }
    attributes
}

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn get_element(node: &Node) -> Element {
    match node {
        IsElement(e) => e.clone(),
        _ => panic!("Node is not an element"),
    }
}
