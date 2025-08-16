use std::rc::Rc;

use pretty_format::PrettyFormatOptions;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, Element, HtmlElement, Node};

use crate::{config::get_config, dom_element_filter::DomElementFilter, helpers::get_document};

pub enum DocumentOrElement {
    Document(Document),
    Element(Element),
}

impl From<Document> for DocumentOrElement {
    fn from(value: Document) -> Self {
        Self::Document(value)
    }
}

impl From<Element> for DocumentOrElement {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}

impl From<HtmlElement> for DocumentOrElement {
    fn from(value: HtmlElement) -> Self {
        Self::Element(value.into())
    }
}

fn should_highlight() -> bool {
    // TODO

    // Don't colorize in non-node environments (e.g. browsers).
    false
}

fn filter_comments_and_default_ignore_tags_tags(node: &Node) -> bool {
    node.node_type() != Node::COMMENT_NODE
        && (node.node_type() != Node::ELEMENT_NODE
            || !node
                .unchecked_ref::<Element>()
                .matches(&get_config().default_ignore)
                .unwrap_or(false))
}

pub fn pretty_dom(dom: Option<DocumentOrElement>, max_length: Option<usize>) -> String {
    let dom = dom.unwrap_or_else(|| {
        get_document()
            .body()
            .expect("Body should exist.")
            .unchecked_into::<Element>()
            .into()
    });
    let max_length = max_length.unwrap_or(7000);

    if max_length == 0 {
        return "".to_owned();
    }

    let dom: JsValue = match dom {
        DocumentOrElement::Document(document) => match document.document_element() {
            Some(element) => element.unchecked_into(),
            None => document.unchecked_into(),
        },
        DocumentOrElement::Element(element) => element.unchecked_into(),
    };

    // TODO: accept as option
    let filter_node = filter_comments_and_default_ignore_tags_tags;

    let debug_content = pretty_format::format(
        &dom,
        // TODO: pass options
        PrettyFormatOptions::default()
            .plugins(vec![Rc::new(DomElementFilter::new(Box::new(filter_node)))])
            .print_function_name(false)
            .highlight(should_highlight()),
    )
    .expect("TODO: return result from pretty_dom()");

    if debug_content.len() > max_length {
        format!("{}...", &debug_content[0..max_length])
    } else {
        debug_content
    }
}

pub fn log_dom(dom: Option<DocumentOrElement>, max_length: Option<usize>) {
    // TODO: User code frame.
    log::info!("{}", pretty_dom(dom, max_length));
}
