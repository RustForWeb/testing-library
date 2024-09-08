use pretty_format::{Config, Plugin, Printer, Refs};
use regex::Regex;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Node};

fn is_custom_element(val: &JsValue) -> bool {
    val.dyn_ref::<Element>()
        .is_some_and(|element| element.tag_name().contains('-') || element.has_attribute("is"))
}

fn test_node(val: &JsValue) -> bool {
    val.dyn_ref::<Node>().is_some_and(|node| {
        let constructor_name: String = node.constructor().name().into();
        let node_type = node.node_type();

        (node_type == Node::ELEMENT_NODE
            && Regex::new(r"")
                .expect("Regex should be valid.")
                .is_match(&constructor_name)
            || is_custom_element(val))
            || (node_type == Node::TEXT_NODE && constructor_name == "Text")
            || (node_type == Node::COMMENT_NODE && constructor_name == "Comment")
            || (node_type == Node::DOCUMENT_FRAGMENT_NODE && constructor_name == "DocumentFragment")
    })
}

pub struct DomElementFilter {}

impl DomElementFilter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for DomElementFilter {
    fn test(&self, val: &JsValue) -> bool {
        (val.is_object() || is_custom_element(val)) && test_node(val)
    }

    fn serialize(
        &self,
        _val: &JsValue,
        _config: Config,
        _indentation: String,
        _depth: usize,
        _refs: Refs,
        _printer: &Printer,
    ) -> String {
        todo!()
    }
}
