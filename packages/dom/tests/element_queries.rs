// TODO: Enable `cfg`, disable `expect`.
// #![cfg(target_arch = "wasm32")]
#![allow(dead_code)]

mod helpers;

use std::sync::{Arc, LazyLock, Mutex};

use indoc::indoc;
use regex::Regex;
use testing_library_dom::{
    ConfigFnOrPartial, MatcherOptions, PartialConfig, QueryError, SelectorMatcherOptions, configure,
};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::{RenderReturn, document, render};

wasm_bindgen_test_configure!(run_in_browser);

static ORIGINAL_CONFIG: LazyLock<Arc<Mutex<PartialConfig>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PartialConfig::default())));

fn before_each() {
    configure(ConfigFnOrPartial::Fn(Box::new(|existing_config| {
        // Grab the existing configuration so we can restore it at the end of the test.
        let mut original_config = ORIGINAL_CONFIG
            .lock()
            .expect("Original config mutex should be acquired.");
        *original_config = PartialConfig::from(existing_config);

        // Don't change the existing config.
        PartialConfig::default()
    })));
}

fn after_each() {
    let original_config = ORIGINAL_CONFIG
        .lock()
        .expect("Original config mutex should be acquired.");

    configure(ConfigFnOrPartial::Partial((*original_config).clone()));
}

#[wasm_bindgen_test]
fn query_can_return_none() -> Result<(), QueryError> {
    before_each();

    let RenderReturn {
        container_queries, ..
    } = render("<div />", None);

    assert!(
        container_queries
            .query_by_test_id("LucyRicardo", MatcherOptions::default())?
            .is_none()
    );
    assert!(
        container_queries
            .query_by_label_text("LucyRicardo", SelectorMatcherOptions::default())?
            .is_none()
    );
    assert!(
        container_queries
            .query_by_display_value("LucyRicardo", MatcherOptions::default())?
            .is_none()
    );
    assert!(
        container_queries
            .query_by_placeholder_text("LucyRicardo", MatcherOptions::default())?
            .is_none()
    );
    assert!(
        container_queries
            .query_by_text("LucyRicardo", SelectorMatcherOptions::default())?
            .is_none()
    );
    assert!(
        container_queries
            .query_by_alt_text("LucyRicardo", MatcherOptions::default())?
            .is_none()
    );

    after_each();

    Ok(())
}

#[wasm_bindgen_test]
fn get_throws_a_useful_error_message() -> Result<(), QueryError> {
    before_each();

    let RenderReturn {
        container_queries, ..
    } = render(
        "<div></div><!-- Ignored comment --><style type=\"text/css\">body {} </style><script type=\"text/javascript\"></script>",
        None,
    );

    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find a label with the text of: LucyRicardo

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_label_text("LucyRicardo", SelectorMatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element with the placeholder text: LucyRicardo

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_placeholder_text("LucyRicardo", MatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element with the text: LucyRicardo. This could be because the text is broken up by multiple elements. In this case, you can provide a function for your text matcher to make your matcher more flexible.

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_text("LucyRicardo", SelectorMatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element with the text: Lucy Ricardo (normalized from 'Lucy      Ricardo'). This could be because the text is broken up by multiple elements. In this case, you can provide a function for your text matcher to make your matcher more flexible.

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_text("Lucy      Ricardo", SelectorMatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element with the text: LucyRicardo, which matches selector 'span'. This could be because the text is broken up by multiple elements. In this case, you can provide a function for your text matcher to make your matcher more flexible.

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_text("LucyRicardo", SelectorMatcherOptions::default().selector("span"))
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element by: [data-testid=\"LucyRicardo\"]

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_test_id("LucyRicardo", MatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element with the alt text: LucyRicardo

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_alt_text("LucyRicardo", MatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element with the title: LucyRicardo

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_title("LucyRicardo", MatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            indoc! {"
            Unable to find an element with the display value: LucyRicardo

            Ignored nodes: comments, script, style
            <div>
              <div />
            </div>"}
            .into()
        )),
        container_queries.get_by_display_value("LucyRicardo", MatcherOptions::default())
    );
    // TODO
    // assert_eq!(
    //     Err(QueryError::Element(
    //         indoc! {"
    //         Unable to find an accessible element with the role: \"LucyRicardo\"

    //         There are no accessible roles. But there might be some inaccessible roles. If you wish to access them, then set the `hidden` option to `true`. Learn more about this here: https://testing-library.com/docs/dom-testing-library/api-queries#byrole

    //         Ignored nodes: comments, script, style
    //         <div>
    //           <div />
    //         </div>"}
    //         .into()
    //     )),
    //     container_queries.get_by_role("LucyRicardo", ByRoleOptions::default())
    // );

    after_each();

    Ok(())
}

#[wasm_bindgen_test]
fn can_get_elements_by_matching_their_text_content() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <div>
            <span>Currently showing</span>
            <span>
              Step
              1
                of 4
            </span>
          </div>
        "},
        None,
    );

    assert!(
        container_queries
            .query_by_text("Currently showing", SelectorMatcherOptions::default())
            .ok()
            .is_some()
    );
    assert!(
        container_queries
            .query_by_text("Step 1 of 4", SelectorMatcherOptions::default())
            .ok()
            .is_some()
    );
}

#[wasm_bindgen_test]
fn can_get_elements_by_matching_their_text_across_adjacent_text_nodes() {
    let document = document();

    let text_div = document
        .create_element("div")
        .expect("Element should be created.");
    let text_node_content = ["£", "24", ".", "99"];

    for text in text_node_content {
        let text_node = document.create_text_node(text);
        text_div
            .append_child(&text_node)
            .expect("Child should be appended.");
    }

    let RenderReturn {
        container,
        container_queries,
        ..
    } = render("<div />", None);

    container
        .append_child(&text_div)
        .expect("Child should be appended.");

    assert!(
        container_queries
            .query_by_text("£24.99", SelectorMatcherOptions::default())
            .ok()
            .is_some()
    );
}

#[wasm_bindgen_test]
fn can_get_input_elements_with_type_submit_or_button_or_reset() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <div>
            <input type=\"submit\" value=\"Send data\"/>
            <input type=\"reset\" value=\"Clear EVERYTHING\"/>
            <input type=\"button\" value=\"Push me!\"/>
            <input type=\"text\" value=\"user data\" />
          </div>
        "},
        None,
    );

    assert!(
        container_queries
            .query_by_text("Send data", SelectorMatcherOptions::default())
            .expect("Query should succeed.")
            .is_some()
    );
    assert!(
        container_queries
            .query_by_text("Clear EVERYTHING", SelectorMatcherOptions::default())
            .expect("Query should succeed.")
            .is_some()
    );
    assert!(
        container_queries
            .query_by_text("Push me!", SelectorMatcherOptions::default())
            .expect("Query should succeed.")
            .is_some()
    );
    assert!(
        container_queries
            .query_by_text("user data", SelectorMatcherOptions::default())
            .expect("Query should succeed.")
            .is_none()
    );
}

#[wasm_bindgen_test]
fn matches_case_with_regexp_matcher() {
    let RenderReturn {
        container_queries, ..
    } = render("<span>STEP 1 of 4</span>", None);

    assert!(
        container_queries
            .query_by_text(
                Regex::new("STEP 1 of 4").expect("Regex should be valid."),
                SelectorMatcherOptions::default()
            )
            .expect("Query should succeed.")
            .is_some()
    );
    assert!(
        container_queries
            .query_by_text(
                Regex::new("Step 1 of 4").expect("Regex should be valid."),
                SelectorMatcherOptions::default()
            )
            .expect("Query should succeed.")
            .is_none()
    );
}

#[wasm_bindgen_test]
fn query_by_text_matches_case_with_non_string_matcher() {
    let RenderReturn {
        container_queries, ..
    } = render("<span>1</span>", None);

    assert!(
        container_queries
            .query_by_text(1_f64, SelectorMatcherOptions::default())
            .expect("Query should succeed.")
            .is_some()
    );
}

#[ignore = "TODO: Fix failing test."]
#[wasm_bindgen_test]
fn can_get_form_controls_by_label_text() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
        <div>
          <label>
            1st<input id=\"first-id\" />
          </label>
          <div>
            <label for=\"second-id\">2nd</label>
            <input id=\"second-id\" />
          </div>
          <div>
            <label id=\"third-label\">3rd</label>
            <input aria-labelledby=\"third-label\" id=\"third-id\" />
          </div>
          <div>
            <label for=\"fourth.id\">4th</label>
            <input id=\"fourth.id\" />
          </div>
          <div>
          <div>
            <label id=\"fifth-label-one\">5th one</label>
            <label id=\"fifth-label-two\">5th two</label>
            <input aria-labelledby=\"fifth-label-one fifth-label-two\" id=\"fifth-id\" />
          </div>
          <div>
            <input id=\"sixth-label-one\" value=\"6th one\"/>
            <input id=\"sixth-label-two\" value=\"6th two\"/>
            <label id=\"sixth-label-three\">6th three</label>
            <input aria-labelledby=\"sixth-label-one sixth-label-two sixth-label-three\" id=\"sixth-id\" />
          </div>
          <div>
            <span id=\"seventh-label-one\">7th one</span>
            <input aria-labelledby=\"seventh-label-one\" id=\"seventh-id\" />
          </div>
          <div>
            <label id=\"eighth.label\">8th one</label>
            <input aria-labelledby=\"eighth.label\" id=\"eighth.id\" />
          </div>
        </div>
      "},
        None,
    );

    assert_eq!(
        container_queries
            .get_by_label_text("1st", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("first-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("2nd", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("second-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("3rd", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("third-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("4th", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("fourth-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("5th one", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("fifth-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("5th two", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("fifth-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("6th one", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("sixth-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("6th two", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("sixth-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("6th one 6th two", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("sixth-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text(
                "6th one 6th two 6th three",
                SelectorMatcherOptions::default()
            )
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("sixth-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("7th one", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("seventh-id".to_owned())
    );
    assert_eq!(
        container_queries
            .get_by_label_text("8th one", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("eighth.id".to_owned())
    );
}

#[wasm_bindgen_test]
fn can_get_elements_labelled_with_aria_labelledby_attribute() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <div>
            <h1 id=\"content-header\">The Gettysburg Address</h1>
            <main id=\"sibling-of-content-header\" aria-labelledby=\"content-header\">
              <section aria-labelledby=\"content-header section-one-header\" id=\"section-one\">
                <h2 id=\"section-one-header\">Section One</h2>
                <p>Four score and seven years ago, ...</p>
              </section>
            </main>
            <p>The Gettysburg Address</p>
          </div>
        "},
        None,
    );

    let result = container_queries
        .get_all_by_label_text("The Gettysburg Address", SelectorMatcherOptions::default())
        .expect("Get should succeed.")
        .into_iter()
        .map(|element| element.id())
        .collect::<Vec<_>>();

    assert_eq!(result.len(), 2);
    assert_eq!(result, vec!["sibling-of-content-header", "section-one"]);
    assert_eq!(
        container_queries
            .get_by_label_text("Section One", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.id()),
        Some("section-one".to_owned())
    );
}

#[wasm_bindgen_test]
fn can_get_sibling_elements_with_aria_labelledby_attribute() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <div>
            <svg id=\"icon\" aria-labelledby=\"icon-desc\"></svg>
            <span id=\"icon-desc\">Tacos</span>
          </div>
        "},
        None,
    );

    let result = container_queries
        .get_all_by_label_text("Tacos", SelectorMatcherOptions::default())
        .expect("Get should succeed.");

    assert_eq!(result.len(), 1);
    assert_eq!(
        result.first().map(|element| element.id()),
        Some("icon".to_owned())
    );
}

#[ignore = "TODO: Fix failing test."]
#[wasm_bindgen_test]
fn can_filter_results_of_label_query_based_on_selector() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <div>
            <label id=\"label1\" for=\"input1\">
              Test Label
              <input id=\"input2\" />
            </label>
            <input id=\"input1\" class=\"fancy-input\" />
            <span aria-labelledby=\"label1\">Some hint text</span>
          </div>
        "},
        None,
    );

    let result = container_queries
        .get_all_by_label_text(
            "Test Label",
            SelectorMatcherOptions::default().selector(".fancy-input"),
        )
        .expect("Get should succeed.");

    assert_eq!(result.len(), 1);
    assert_eq!(
        result.first().map(|element| element.id()),
        Some("input1".to_owned())
    );
}

#[ignore = "TODO: Fix failing test."]
#[wasm_bindgen_test]
fn can_find_any_labelable_element_when_label_text_is_inside_other_elements() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <label>
            <span>Test</span>
            <span>Label</span>
            <button />
            <input />
            <meter />
            <output />
            <progress />
            <select />
            <textarea />
          </label>
        "},
        None,
    );

    let node_types = [
        "button", "input", "meter", "output", "progress", "select", "textarea",
    ];
    for node_type in node_types {
        assert_eq!(
            container_queries
                .get_by_label_text(
                    "Test Label",
                    SelectorMatcherOptions::default().selector(node_type)
                )
                .expect("Get should succeed.")
                .map(|element| element.node_name()),
            Some(node_type.to_uppercase())
        );
    }
}

// According to the spec, the first descendant of a label that is a labelable element is the labeled control.
// https://html.spec.whatwg.org/multipage/forms.html#the-label-element
#[ignore = "TODO: Fix failing test."]
#[wasm_bindgen_test]
fn returns_the_labelable_element_control_inside_a_label() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <label>
            <span>Test</span>
            <span>Label</span>
            <button />
            <input />
            <meter />
            <output />
            <progress />
            <select />
            <textarea ></textarea>
          </label>
        "},
        None,
    );

    assert_eq!(
        container_queries
            .get_by_label_text("Test Label", SelectorMatcherOptions::default())
            .expect("Get should succeed.")
            .map(|element| element.node_name()),
        Some("BUTTON".to_owned())
    );
}

#[ignore = "TODO: Fix failing test."]
#[wasm_bindgen_test]
fn can_find_non_input_elements_when_aria_labelledby_a_label() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <div>
            <label id=\"label1\">Test Label</label>
            <ul aria-labelledby=\"label1\">
              <li>Hello</li>
            </ul
          </div>
        "},
        None,
    );

    let result = container_queries
        .get_all_by_label_text("Test Label", SelectorMatcherOptions::default())
        .expect("Get should succeed.");

    assert_eq!(result.len(), 1);
    assert_eq!(
        result.first().map(|element| element.node_name()),
        Some("UL".to_owned())
    );
}

#[ignore = "TODO: Fix failing test."]
#[wasm_bindgen_test]
fn can_find_the_correct_element_when_there_are_multiple_matching_labels() {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <label>
            Test Label
            <input />
          </label>
          <label>
            Test Label
            <textarea></textarea>
          </label>
        "},
        None,
    );

    let result = container_queries
        .get_by_label_text(
            "Test Label",
            SelectorMatcherOptions::default().selector("input"),
        )
        .expect("Get should succeed.");

    assert_eq!(
        result.map(|element| element.node_name()),
        Some("INPUT".to_owned())
    );
}

#[wasm_bindgen_test]
fn query_by_placeholder_text_matches_case_with_non_string_matcher() {
    let RenderReturn {
        container_queries, ..
    } = render("<input placeholder=\"1\" />", None);

    assert!(
        container_queries
            .query_by_placeholder_text(1_f64, MatcherOptions::default())
            .expect("Query should succeed.")
            .is_some()
    );
}

// TODO: More tests.
