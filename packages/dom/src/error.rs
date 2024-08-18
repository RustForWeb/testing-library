use thiserror::Error;
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("invalid configuration: {0}")]
    Configuration(String),
    #[error("{0:?}")]
    JsError(JsValue),
    #[error("{0}")]
    NoElements(String),
    #[error("{0}")]
    MultipleElements(String),
    #[error("{0}")]
    Element(String),
}
