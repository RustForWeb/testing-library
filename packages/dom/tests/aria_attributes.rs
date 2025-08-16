// TODO: Enable `cfg`, disable `expect`.
// #![cfg(target_arch = "wasm32")]
#![allow(dead_code)]

mod helpers;

use indoc::indoc;
use testing_library_dom::{AriaRole, ByRoleOptions, QueryError, SelectorMatcherOptions};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::HtmlInputElement;

use crate::helpers::test_utils::render_into_document_div;

use self::helpers::test_utils::{RenderReturn, render};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn selected_throws_on_unsupported_roles() {
    let RenderReturn {
        container_queries, ..
    } = render("<input aria-selected=\"true\" type=\"text\">", None);

    assert_eq!(
        container_queries.get_by_role(AriaRole::Textbox, ByRoleOptions::default().selected(true)),
        Err(QueryError::Unsupported(
            "\"aria-selected\" is not supported on role \"textbox\".".to_owned()
        ))
    );
}

#[wasm_bindgen_test]
fn pressed_throws_on_unsupported_roles() {
    let RenderReturn {
        container_queries, ..
    } = render("<input aria-pressed=\"true\" type=\"text\">", None);

    assert_eq!(
        container_queries.get_by_role(AriaRole::Textbox, ByRoleOptions::default().pressed(true)),
        Err(QueryError::Unsupported(
            "\"aria-pressed\" is not supported on role \"textbox\".".to_owned()
        ))
    );
}

#[wasm_bindgen_test]
fn checked_throws_on_unsupported_roles() {
    let RenderReturn {
        container_queries, ..
    } = render("<input aria-checked=\"true\" type=\"text\">", None);

    assert_eq!(
        container_queries.get_by_role(AriaRole::Textbox, ByRoleOptions::default().checked(true)),
        Err(QueryError::Unsupported(
            "\"aria-checked\" is not supported on role \"textbox\".".to_owned()
        ))
    );
}

#[wasm_bindgen_test]
fn expanded_throws_on_unsupported_roles() {
    let RenderReturn {
        container_queries, ..
    } = render("<h1 aria-expanded=\"true\">Heading</h1>", None);

    assert_eq!(
        container_queries.get_by_role(AriaRole::Heading, ByRoleOptions::default().expanded(true)),
        Err(QueryError::Unsupported(
            "\"aria-expanded\" is not supported on role \"heading\".".to_owned()
        ))
    );
}

#[wasm_bindgen_test]
fn busy_throws_on_unsupported_roles() {
    let RenderReturn {
        container_queries, ..
    } = render(
        "<div aria-busy=\"true\" role=\"none\">Hello, Dave!</div>",
        None,
    );

    assert_eq!(
        container_queries.get_by_role(AriaRole::None, ByRoleOptions::default().busy(true)),
        Err(QueryError::Unsupported(
            "\"aria-busy\" is not supported on role \"none\".".to_owned()
        ))
    );
}

#[wasm_bindgen_test]
fn busy_true_false_matches_busy_regions() {
    let RenderReturn {
        container_queries, ..
    } = render_into_document_div(indoc! {"
      <div>
        <div role=\"log\" aria-busy=\"true\" />
        <div role=\"log\" aria-busy=\"false\" />
      </div>
    "});

    assert!(
        container_queries
            .get_by_role(AriaRole::Log, ByRoleOptions::default().busy(true))
            .is_ok()
    );
    assert!(
        container_queries
            .get_by_role(AriaRole::Log, ByRoleOptions::default().busy(false))
            .is_ok()
    );
}

#[wasm_bindgen_test]
fn checked_true_false_matches_checked_checkboxes() {
    let RenderReturn {
        container_queries, ..
    } = render_into_document_div(indoc! {"
      <div>
        <input type=\"checkbox\" checked />
        <input type=\"checkbox\" />
      </div>
    "});

    assert!(
        container_queries
            .get_by_role(AriaRole::Checkbox, ByRoleOptions::default().checked(true))
            .is_ok()
    );
    assert!(
        container_queries
            .get_by_role(AriaRole::Checkbox, ByRoleOptions::default().checked(false))
            .is_ok(),
    );
}

#[wasm_bindgen_test]
fn checked_true_false_matches_checked_elements_with_proper_role() {
    let RenderReturn {
        container_queries, ..
    } = render_into_document_div(indoc! {"
      <div>
        <span role=\"checkbox\" aria-checked=\"true\">✔</span>
        <span role=\"checkbox\" aria-checked=\"false\">𝒙</span>
      </div>
    "});

    assert!(
        container_queries
            .get_by_role(AriaRole::Checkbox, ByRoleOptions::default().checked(true))
            .is_ok()
    );
    assert!(
        container_queries
            .get_by_role(AriaRole::Checkbox, ByRoleOptions::default().checked(false))
            .is_ok()
    );
}

#[wasm_bindgen_test]
fn checked_true_false_does_not_match_element_in_indeterminate_state() {
    let RenderReturn {
        container_queries, ..
    } = render_into_document_div(indoc! {"
      <div>
        <span role=\"checkbox\" aria-checked=\"mixed\">not so much</span>
        <input type=\"checkbox\" checked aria-label=\"indeteminate yes\" />
        <input type=\"checkbox\" aria-label=\"indeteminate no\" />
      </div>
    "});

    container_queries
        .get_by_label_text("indeteminate yes", SelectorMatcherOptions::default())
        .expect("Get should succeed.")
        .unchecked_into::<HtmlInputElement>()
        .set_indeterminate(true);
    container_queries
        .get_by_label_text("indeteminate no", SelectorMatcherOptions::default())
        .expect("Get should succeed.")
        .unchecked_into::<HtmlInputElement>()
        .set_indeterminate(true);

    assert!(
        container_queries
            .query_by_role(
                AriaRole::Checkbox,
                ByRoleOptions::default()
                    .checked(true)
                    .name("indeteminate yes")
            )
            .expect("Get should succeed.")
            .is_none(),
    );
    assert!(
        container_queries
            .query_by_role(
                AriaRole::Checkbox,
                ByRoleOptions::default()
                    .checked(false)
                    .name("indeteminate no")
            )
            .expect("Get should succeed.")
            .is_none(),
    );
    assert!(
        container_queries
            .query_by_role(
                AriaRole::Checkbox,
                ByRoleOptions::default().checked(true).name("not so much")
            )
            .expect("Get should succeed.")
            .is_none(),
    );
}

// TODO: More tests.
