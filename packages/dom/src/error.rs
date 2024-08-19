use thiserror::Error;
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug, Error, PartialEq)]
pub enum QueryError {
    #[error("invalid configuration: {0}")]
    Configuration(String),
    #[error("{0:?}")]
    JsError(JsValue),
    #[error("{0}")]
    Element(String),
}
