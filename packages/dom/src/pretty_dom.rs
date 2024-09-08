use std::rc::Rc;

use pretty_format::PrettyFormatOptions;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, Element};

use crate::{dom_element_filter::DomElementFilter, helpers::get_document};

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

fn should_highlight() -> bool {
    // TODO
    true
}

pub fn pretty_dom(dom: Option<DocumentOrElement>, max_length: Option<usize>) -> String {
    let dom = dom.unwrap_or_else(|| get_document().into());
    let max_length = max_length.unwrap_or(7000);

    if max_length == 0 {
        return "".into();
    }

    let dom: JsValue = match dom {
        DocumentOrElement::Document(document) => match document.document_element() {
            Some(element) => element.unchecked_into(),
            None => document.unchecked_into(),
        },
        DocumentOrElement::Element(element) => element.unchecked_into(),
    };

    let debug_content = pretty_format::format(
        &dom,
        // TODO: pass options
        PrettyFormatOptions::default()
            .plugins(vec![Rc::new(DomElementFilter::new())])
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
