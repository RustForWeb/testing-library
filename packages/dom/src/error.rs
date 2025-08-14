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
    #[error("{0}")]
    Unsupported(String),
}

#[derive(Debug, Error, PartialEq)]
pub enum FireEventError {
    #[error("{0:?}")]
    JsError(JsValue),
}

#[derive(Debug, Error, PartialEq)]
pub enum CreateEventError {
    #[error("{0:?}")]
    JsError(JsValue),
}

#[derive(Debug, Error, PartialEq)]
pub enum CreateOrFireEventError {
    #[error(transparent)]
    Create(#[from] CreateEventError),
    #[error(transparent)]
    Fire(#[from] FireEventError),
}
