use std::borrow::Cow;
use std::collections::HashMap;

use html5ever::tendril::*;
use html5ever::tree_builder::{
    AppendNode, AppendText, ElementFlags, NodeOrText, QuirksMode, TreeBuilderOpts, TreeSink,
};
use html5ever::{
    expanded_name, local_name, namespace_url, ns, Attribute, ExpandedName, ParseOpts, QualName,
};

pub struct Sink {
    pub next_id: usize,
    pub names: HashMap<usize, QualName>,
    pub line: u64,
}

impl Sink {
    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 2;
        id
    }
}

impl TreeSink for Sink {
    type Handle = usize;
    type Output = Self;

    fn finish(self) -> Self {
        println!("done");
        self
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        match msg {
            Cow::Borrowed("Bad DOCTYPE") => println!("Not bad doctype"),
            _ => {
                eprintln!("Error Parsing on line {}", self.line - 1);
                std::process::exit(1)
            }
        }
    }

    fn get_document(&mut self) -> usize {
        0
    }

    fn get_template_contents(&mut self, target: &usize) -> usize {
        if let Some(expanded_name!(html "template")) = self.names.get(target).map(|n| n.expanded())
        {
            target + 1
        } else {
            panic!("not a template element")
        }
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        println!("Set quirks mode to {:?}", mode);
    }

    fn same_node(&self, x: &usize, y: &usize) -> bool {
        x == y
    }

    fn elem_name(&self, target: &usize) -> ExpandedName {
        self.names.get(target).expect("not an element").expanded()
    }

    fn create_element(
        &mut self,
        name: QualName,
        attributes: Vec<Attribute>,
        _: ElementFlags,
    ) -> usize {
        let id = self.get_id();
        println!("Created {:?} as {} with {:#?}", name, id, attributes);
        self.names.insert(id, name);
        id
    }

    fn create_comment(&mut self, text: StrTendril) -> usize {
        let id = self.get_id();
        println!("Created comment \"{}\" as {}", escape_default(&text), id);
        id
    }

    #[allow(unused_variables)]
    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> usize {
        unimplemented!()
    }

    fn append(&mut self, parent: &usize, child: NodeOrText<usize>) {
        match child {
            AppendNode(n) => println!("Append node {} to {}", n, parent),
            AppendText(t) => println!(
                "Append text to {:#?}: \"{}\"",
                self.names[parent],
                escape_default(&t)
            ),
        }
    }

    fn append_before_sibling(&mut self, sibling: &usize, new_node: NodeOrText<usize>) {
        match new_node {
            AppendNode(n) => println!("Append node {} before {}", n, sibling),
            AppendText(t) => println!("Append text before {}: \"{}\"", sibling, escape_default(&t)),
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        _prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        self.append_before_sibling(element, child);
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        println!("Append doctype: {} {} {}", name, public_id, system_id);
    }

    fn add_attrs_if_missing(&mut self, target: &usize, attrs: Vec<Attribute>) {
        assert!(self.names.contains_key(target), "not an element");
        println!("Add missing attributes to {}:", target);
        for attr in attrs.into_iter() {
            println!("    {:?} = {}", attr.name, attr.value);
        }
    }

    fn associate_with_form(
        &mut self,
        _target: &usize,
        _form: &usize,
        _nodes: (&usize, Option<&usize>),
    ) {
        // No form owner support.
    }

    fn remove_from_parent(&mut self, target: &usize) {
        println!("Remove {} from parent", target);
    }

    fn reparent_children(&mut self, node: &usize, new_parent: &usize) {
        println!("Move children from {} to {}", node, new_parent);
    }

    fn mark_script_already_started(&mut self, node: &usize) {
        println!("Mark script {} as already started", node);
    }

    fn set_current_line(&mut self, line_number: u64) {
        println!("Set current line to {}", line_number);
        self.line = line_number;
    }

    fn pop(&mut self, elem: &usize) {
        println!("Popped element {}", elem);
    }
}

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
