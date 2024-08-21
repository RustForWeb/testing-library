use web_sys::{window, Document};

pub fn get_document() -> Document {
    window()
        .and_then(|window| window.document())
        .expect("Could not find default container")
}
