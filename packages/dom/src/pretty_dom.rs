use web_sys::{Document, Element};

use crate::helpers::get_document;

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

pub fn pretty_dom(dom: Option<DocumentOrElement>, max_length: Option<usize>) -> String {
    let dom = dom.unwrap_or_else(|| get_document().into());
    let max_length = max_length.unwrap_or(7000);

    if max_length == 0 {
        return "".into();
    }

    let _dom = match dom {
        DocumentOrElement::Document(document) => match document.document_element() {
            Some(element) => DocumentOrElement::Element(element),
            None => DocumentOrElement::Document(document),
        },
        dom => dom,
    };

    // TODO
    let debug_content = "".to_string();

    if debug_content.len() > max_length {
        format!("{}...", &debug_content[0..max_length])
    } else {
        debug_content
    }
}
