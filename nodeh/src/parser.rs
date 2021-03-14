use std::borrow::Cow;
use std::collections::HashMap;

use html5ever::tendril::*;
use html5ever::tree_builder::{
    AppendNode, AppendText, ElementFlags, NodeOrText, QuirksMode, TreeSink,
};
use html5ever::{
    expanded_name, local_name, namespace_url, ns, Attribute, ExpandedName, LocalName, QualName,
};

type Handle = usize;

struct Element {
    element_name: String,
    parent: Handle,
    parent_func: Handle,
    qual_name: QualName,
}

#[derive(Default)]
pub struct Parser {
    next_id: Handle,
    line: u64,
    elements: HashMap<Handle, Element>,
}

impl Parser {
    fn get_id(&mut self) -> Handle {
        let id = self.next_id;
        self.next_id += 2;
        id
    }

    pub fn new() -> Parser {
        Parser {
            next_id: 1,
            line: 0,
            elements: HashMap::new(),
        }
    }
}

impl TreeSink for Parser {
    type Handle = usize;
    type Output = Self;

    fn finish(self) -> Self {
        self
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        match msg {
            Cow::Borrowed("Bad DOCTYPE") => {}
            _ => {
                eprintln!("Error Parsing on line {}", self.line);
                std::process::exit(1)
            }
        }
    }

    fn get_document(&mut self) -> Handle {
        0
    }

    fn elem_name(&self, target: &Handle) -> ExpandedName {
        self.elements
            .get(target)
            .expect("No element found")
            .qual_name
            .expanded()
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>, _: ElementFlags) -> Handle {
        let id = self.get_id();
        let element = Element {
            element_name: name.local.to_string(),
            parent: 0,
            parent_func: 0,
            qual_name: name,
        };
        self.elements.insert(id, element);
        id
    }

    fn create_comment(&mut self, _: StrTendril) -> Handle {
        unimplemented!()
    }

    fn create_pi(&mut self, _: StrTendril, _: StrTendril) -> Handle {
        unimplemented!()
    }

    fn append(&mut self, parent: &Handle, child: NodeOrText<Handle>) {}

    fn append_based_on_parent_node(
        &mut self,
        element: &Handle,
        prev_element: &Handle,
        child: NodeOrText<Handle>,
    ) {
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
    }

    fn get_template_contents(&mut self, target: &Handle) -> Handle {
        unimplemented!()
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {}

    fn append_before_sibling(&mut self, sibling: &Handle, new_node: NodeOrText<Handle>) {}

    fn add_attrs_if_missing(&mut self, target: &Handle, attrs: Vec<Attribute>) {}

    fn remove_from_parent(&mut self, target: &Handle) {}

    fn reparent_children(&mut self, node: &Handle, new_parent: &Handle) {}

    fn set_current_line(&mut self, _line_number: u64) {
        self.line = _line_number;
    }
}
