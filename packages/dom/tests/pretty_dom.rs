#![cfg(target_arch = "wasm32")]

mod helpers;

use helpers::test_utils::render_into_document;
use indoc::indoc;
use testing_library_dom::pretty_dom;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::{RenderReturn, render};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pretty_dom_prints_out_the_given_dom_element_tree() {
    let RenderReturn { container, .. } = render("<div>Hello World!</div>", None);

    assert_eq!(
        indoc! {"
        <div>
          <div>
            Hello World!
          </div>
        </div>"},
        pretty_dom(Some(container.into()), None)
    );
}

#[wasm_bindgen_test]
fn pretty_dom_supports_truncating_the_output_length() {
    let RenderReturn { container, .. } = render("<div>Hello World!</div>", None);

    assert_eq!(
        "<div>...",
        pretty_dom(Some(container.clone().into()), Some(5))
    );
    assert_eq!("", pretty_dom(Some(container.clone().into()), Some(0)));
    assert_eq!(
        indoc! {"
        <div>
          <div>
            Hello World!
          </div>
        </div>"},
        pretty_dom(Some(container.into()), Some(usize::MAX))
    );
}

#[ignore = "`wasm-pack test` crashes when its `pre` elements are overwritten."]
#[wasm_bindgen_test]
fn pretty_dom_defaults_to_document_body() {
    render_into_document("<div>Hello World!</div>");

    assert_eq!(
        indoc! {"
        <body>
          <div>
            Hello World!
          </div>
        </body>"},
        pretty_dom(None, None)
    );
}

#[ignore = "`wasm-pack test` crashes when its `pre` elements are overwritten."]
#[wasm_bindgen_test]
fn pretty_dom_supports_receiving_the_document_element() {
    assert_eq!(
        indoc! {"
        <html>
          <head />
          <body />
        </html>"},
        pretty_dom(None, None)
    );
}

// TODO
