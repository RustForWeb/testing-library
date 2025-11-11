// TODO: Enable `cfg`, disable `expect`.
// #![cfg(target_arch = "wasm32")]
#![allow(dead_code)]

use testing_library_dom::{
    QueryError, SelectorMatcherOptions, WaitForError, WaitForOptions, screen, wait_for,
};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn todo() -> Result<(), WaitForError<QueryError>> {
    let screen = screen();

    wait_for(
        || screen.get_by_text("Hello World", SelectorMatcherOptions::default()),
        WaitForOptions::default(),
    )
    .await?;

    Ok(())
}
