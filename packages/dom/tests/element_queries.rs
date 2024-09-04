mod helpers;

use std::sync::{Arc, LazyLock, Mutex};

use testing_library_dom::{
    configure, ConfigFnOrPartial, MatcherOptions, PartialConfig, QueryError,
};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::{render, RenderReturn};

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

    assert!(container_queries
        .query_by_display_value("LucyRicardo", MatcherOptions::default())?
        .is_none());
    assert!(container_queries
        .query_by_alt_text("LucyRicardo", MatcherOptions::default())?
        .is_none());

    after_each();

    Ok(())
}

#[wasm_bindgen_test]
fn get_throws_a_useful_error_message() -> Result<(), QueryError> {
    before_each();

    let RenderReturn {
        container_queries, ..
    } = render("<div></div><!-- Ignored comment --><style type=\"text/css\">body {} </style><script type=\"text/javascript\"></script>", None);

    assert_eq!(
        Err(QueryError::Element(
            "Unable to find an element with the alt text: LucyRicardo\n\
            \n\
            Ignored nodes: comments, script, style\n\
            <div>\n\
            <div />\n\
            </div>"
                .into()
        )),
        container_queries.get_by_alt_text("LucyRicardo", MatcherOptions::default())
    );
    assert_eq!(
        Err(QueryError::Element(
            "Unable to find an element with the display value: LucyRicardo\n\
            \n\
            Ignored nodes: comments, script, style\n\
            <div>\n\
            <div />\n\
            </div>"
                .into()
        )),
        container_queries.get_by_display_value("LucyRicardo", MatcherOptions::default())
    );

    after_each();

    Ok(())
}