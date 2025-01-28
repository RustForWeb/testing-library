#![cfg(target_arch = "wasm32")]

use std::sync::Arc;

use mockall::automock;
use testing_library_dom::{
    configure, get_element_error, ConfigFnOrPartial, PartialConfig, QueryError,
};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::{Document, Element};

wasm_bindgen_test_configure!(run_in_browser);

#[automock]
trait GetElementError {
    fn get_element_error(message: Option<String>, container: Element) -> QueryError;
}

#[wasm_bindgen_test]
fn should_delegate_to_config_get_element_error() {
    let message = Some("test Message".to_string());
    let container = Document::new()
        .expect("Document should be created.")
        .create_element("div")
        .expect("Element should be created");

    let mock = MockGetElementError::get_element_error_context();
    mock.expect()
        .withf_st({
            let message = message.clone();
            let container = container.clone();

            move |m, c| *m == message && *c == container
        })
        .returning(|_, _| QueryError::Element("".into()));

    configure(ConfigFnOrPartial::Partial(
        PartialConfig::default().get_element_error(Arc::new(|message, container| {
            MockGetElementError::get_element_error(message, container)
        })),
    ));

    get_element_error(message, container);
}
