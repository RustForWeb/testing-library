use web_sys::{Document, window};

pub fn get_document() -> Document {
    window()
        .and_then(|window| window.document())
        .expect("Could not find default container")
}
