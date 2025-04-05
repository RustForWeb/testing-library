// TODO: remove
#![allow(dead_code)]

use testing_library_dom::{BoundFunctions, get_queries_for_element};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, window};

pub fn document() -> Document {
    window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

pub struct RenderReturn {
    pub container: Element,
    pub container_queries: BoundFunctions,
    pub rerender: Box<dyn Fn(&str) -> RenderReturn>,
}

pub fn render(html: &str, container: Option<Element>) -> RenderReturn {
    let container = container.unwrap_or_else(|| {
        document()
            .create_element("div")
            .expect("Element should be created.")
    });

    container.set_inner_html(html);

    let container_queries =
        get_queries_for_element(container.clone().unchecked_into::<HtmlElement>());

    RenderReturn {
        container: container.clone(),
        container_queries,
        rerender: Box::new(move |new_html| render(new_html, Some(container.clone()))),
    }
}

pub fn render_into_document(html: &str) -> RenderReturn {
    render(
        html,
        Some(document().body().expect("Body should exist.").into()),
    )
}

pub fn cleanup() {
    document()
        .body()
        .expect("Body should exist.")
        .set_inner_html("");
}
