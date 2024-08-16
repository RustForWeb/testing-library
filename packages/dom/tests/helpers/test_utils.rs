use web_sys::{window, Document, Element};

pub fn document() -> Document {
    window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

pub struct RenderReturn {
    pub container: Element,
    pub rerender: Box<dyn Fn(&str) -> RenderReturn>,
}

pub fn render(html: &str, container: Option<Element>) -> RenderReturn {
    let container = container.unwrap_or_else(|| {
        document()
            .create_element("div")
            .expect("Element should be created.")
    });

    container.set_inner_html(html);

    // TODO: container_queries

    RenderReturn {
        container: container.clone(),
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
