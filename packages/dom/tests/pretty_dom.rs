mod helpers;

use indoc::indoc;
use testing_library_dom::pretty_dom;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::{render, RenderReturn};

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
