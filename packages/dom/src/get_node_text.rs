use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, Node};

use crate::util::node_list_to_vec;

pub fn get_node_text(node: &Node) -> String {
    if node
        .dyn_ref::<Element>()
        .map(|element| {
            element
                .matches("input[type=submit], input[type=button], input[type=reset]")
                .is_ok_and(|value| value)
        })
        .unwrap_or(false)
    {
        node.unchecked_ref::<HtmlInputElement>().value()
    } else {
        node_list_to_vec::<Node>(node.child_nodes())
            .into_iter()
            .filter_map(|child| {
                (child.node_type() == Node::TEXT_NODE)
                    .then(|| child.text_content())
                    .flatten()
            })
            .collect::<Vec<_>>()
            .join("")
    }
}
