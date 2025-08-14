use testing_library_dom::{BoundFunctions, get_queries_for_element};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, window};

pub fn document() -> Document {
    window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

pub struct RenderReturn {
    pub container: HtmlElement,
    pub container_queries: BoundFunctions,
    pub rerender: Box<dyn Fn(&str) -> RenderReturn>,
}

pub fn render(html: &str, container: Option<HtmlElement>) -> RenderReturn {
    let container = container.unwrap_or_else(|| {
        document()
            .create_element("div")
            .expect("Element should be created.")
            .unchecked_into::<HtmlElement>()
    });

    container.set_inner_html(html);

    let container_queries = get_queries_for_element(container.clone());

    RenderReturn {
        container: container.clone(),
        container_queries,
        rerender: Box::new(move |new_html| render(new_html, Some(container.clone()))),
    }
}

pub fn render_into_document(html: &str) -> RenderReturn {
    render(html, Some(document().body().expect("Body should exist.")))
}

pub fn render_into_document_div(html: &str) -> RenderReturn {
    // `wasm-pack test` crashes when its `pre` elements are overwritten, so use a `div` instead.
    let container = document()
        .create_element("div")
        .expect("Element should be created.")
        .unchecked_into::<HtmlElement>();
    document()
        .body()
        .expect("Body should exist.")
        .append_child(&container)
        .expect("Child should be appended.");

    render(html, Some(container))
}

pub fn cleanup() {
    document()
        .body()
        .expect("Body should exist.")
        .set_inner_html("");
}
