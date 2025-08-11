#![cfg(target_arch = "wasm32")]

mod helpers;

use regex::Regex;
use testing_library_dom::{MatcherOptions, query_by_attribute};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::{RenderReturn, render};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_query_by_attribute() {
    let RenderReturn { container, .. } = render(
        "<div data-foo=\"bar\"></div><div data-foo=\"rubar\"></div>",
        None,
    );

    assert!(
        query_by_attribute("data-foo", &container, "bar", MatcherOptions::default())
            .expect("Query should succeed.")
            .is_some()
    );
    assert!(
        query_by_attribute("blah", &container, "sup", MatcherOptions::default())
            .expect("Query should succeed.")
            .is_none()
    );
    assert!(
        query_by_attribute(
            "data-foo",
            &container,
            Regex::new("bar").expect("Regex should be valid."),
            MatcherOptions::default()
        )
        .is_err_and(|err| err.to_string().contains("multiple"))
    );
}
