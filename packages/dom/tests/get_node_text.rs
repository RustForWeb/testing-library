#![cfg(target_arch = "wasm32")]

mod helpers;

use testing_library_dom::get_node_text;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::{RenderReturn, render};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn it_prints_out_the_text_content_of_a_container() {
    let RenderReturn { container, .. } = render("Hello <!--skipped-->World!", None);

    assert_eq!("Hello World!", get_node_text(&container));
}

#[wasm_bindgen_test]
fn it_prints_out_the_value_for_submit_and_button_inputs() {
    let RenderReturn { container, .. } = render(
        "<input type=\"submit\" value=\"save\"><input type=\"button\" value=\"reset\">",
        None,
    );

    assert_eq!(
        "save",
        get_node_text(&container.first_child().expect("First child should exist."))
    );

    assert_eq!(
        "reset",
        get_node_text(&container.last_child().expect("Last child should exist."))
    );
}
