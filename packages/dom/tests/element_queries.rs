#![cfg(target_arch = "wasm32")]

mod helpers;

use std::sync::{Arc, LazyLock, Mutex};

use indoc::indoc;
use testing_library_dom::{
    ConfigFnOrPartial, MatcherOptions, PartialConfig, QueryError, SelectorMatcherOptions, configure,
};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::{RenderReturn, render};

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
        container_queries.get_by_text("LucyRicardo", SelectorMatcherOptions::default().selector("span".into()))
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
