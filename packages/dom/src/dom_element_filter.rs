use std::collections::HashMap;

use pretty_format::{Config, Plugin, Printer, Refs};
use regex::Regex;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Comment, Element, Node, Text};

use crate::util::{named_node_map_to_hashmap, node_list_to_vec};

fn escape_html(text: String) -> String {
    text.replace('<', "&lt;").replace('>', "&gt;")
}

fn print_props(
    attributes: HashMap<String, String>,
    config: &Config,
    indentation: String,
    depth: usize,
    refs: Refs,
    printer: &Printer,
) -> String {
    let indentation_next = format!("{}{}", indentation, config.indent);

    attributes
        .into_iter()
        .map(|(key, value)| {
            let printed = printer(
                &JsValue::from_str(&value),
                config,
                indentation_next.clone(),
                depth,
                refs.clone(),
                None,
            );

            format!(
                "{}{}{}={}",
                config.spacing_inner,
                indentation,
                config.colors.prop.paint(&key),
                config.colors.value.paint(&printed)
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

fn print_children(
    children: Vec<Node>,
    config: &Config,
    indentation: String,
    depth: usize,
    refs: Refs,
    printer: &Printer,
) -> String {
    children
        .into_iter()
        .map(|child| {
            let printed_child = printer(
                child.unchecked_ref::<JsValue>(),
                config,
                indentation.clone(),
                depth,
                refs.clone(),
                None,
            );

            if printed_child.is_empty() && child.node_type() != Node::TEXT_NODE {
                // A plugin serialized this Node to '' meaning we should ignore it.
                "".into()
            } else {
                format!("{}{}{}", config.spacing_outer, indentation, printed_child)
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn print_text(text: String, config: &Config) -> String {
    let content_color = &config.colors.content;
    content_color.paint(&escape_html(text))
}

fn print_comment(text: String, config: &Config) -> String {
    let comment_color = &config.colors.comment;
    comment_color.paint(&format!("<!--{}-->", escape_html(text)))
}

fn print_element(
    r#type: String,
    printed_props: String,
    printed_children: String,
    config: &Config,
    indentation: String,
) -> String {
    let tag_color = &config.colors.tag;

    tag_color.paint(&format!(
        "<{}{}{}>",
        r#type,
        if printed_props.is_empty() {
            "".into()
        } else {
            format!(
                "{}{}{}{}{}",
                tag_color.close(),
                printed_props,
                config.spacing_outer,
                indentation,
                tag_color.open()
            )
        },
        if printed_children.is_empty() {
            if !printed_props.is_empty() && !config.min {
                "/".into()
            } else {
                " /".into()
            }
        } else {
            format!(
                ">{}{}{}{}{}</{}",
                tag_color.close(),
                printed_children,
                config.spacing_outer,
                indentation,
                tag_color.open(),
                r#type
            )
        }
    ))
}

fn print_element_as_leaf(r#type: String, config: &Config) -> String {
    let tag_color = &config.colors.tag;
    format!(
        "{} …{}",
        tag_color.paint(&format!("<{type}")),
        tag_color.paint(" />")
    )
}

fn is_custom_element(val: &JsValue) -> bool {
    val.dyn_ref::<Element>()
        .is_some_and(|element| element.tag_name().contains('-') || element.has_attribute("is"))
}

fn test_node(val: &JsValue) -> bool {
    val.dyn_ref::<Node>().is_some_and(|node| {
        let constructor_name: String = node.constructor().name().into();
        let node_type = node.node_type();

        (node_type == Node::ELEMENT_NODE
            && Regex::new(r"^((HTML|SVG)\w*)?Element$")
                .expect("Regex should be valid.")
                .is_match(&constructor_name)
            || is_custom_element(val))
            || (node_type == Node::TEXT_NODE && constructor_name == "Text")
            || (node_type == Node::COMMENT_NODE && constructor_name == "Comment")
            || (node_type == Node::DOCUMENT_FRAGMENT_NODE && constructor_name == "DocumentFragment")
    })
}

fn node_is_text(node: &Node) -> bool {
    node.node_type() == Node::TEXT_NODE
}

fn node_is_comment(node: &Node) -> bool {
    node.node_type() == Node::COMMENT_NODE
}

fn node_is_fragment(node: &Node) -> bool {
    node.node_type() == Node::DOCUMENT_FRAGMENT_NODE
}

pub struct DomElementFilter {
    filter_node: Box<dyn Fn(&Node) -> bool>,
}

impl DomElementFilter {
    pub fn new(filter_node: Box<dyn Fn(&Node) -> bool>) -> Self {
        Self { filter_node }
    }
}

impl Plugin for DomElementFilter {
    fn test(&self, val: &JsValue) -> bool {
        (val.is_object() || is_custom_element(val)) && test_node(val)
    }

    fn serialize(
        &self,
        val: &JsValue,
        config: &Config,
        indentation: String,
        depth: usize,
        refs: Refs,
        printer: &Printer,
    ) -> String {
        let node: &Node = val.unchecked_ref();

        if node_is_text(node) {
            return print_text(node.unchecked_ref::<Text>().data(), config);
        }

        if node_is_comment(node) {
            return print_comment(node.unchecked_ref::<Comment>().data(), config);
        }

        let r#type = if node_is_fragment(node) {
            "DocumentFragment".into()
        } else {
            node.unchecked_ref::<Element>().tag_name().to_lowercase()
        };

        let depth = depth + 1;
        if depth > config.max_depth {
            return print_element_as_leaf(r#type, config);
        }

        print_element(
            r#type,
            print_props(
                if node_is_fragment(node) {
                    HashMap::new()
                } else {
                    named_node_map_to_hashmap(node.unchecked_ref::<Element>().attributes())
                },
                config,
                format!("{}{}", indentation, &config.indent),
                depth,
                refs.clone(),
                printer,
            ),
            print_children(
                node_list_to_vec(node.child_nodes())
                    .into_iter()
                    .filter(&self.filter_node)
                    .collect(),
                config,
                format!("{}{}", indentation, &config.indent),
                depth,
                refs.clone(),
                printer,
            ),
            config,
            indentation,
        )
    }
}
