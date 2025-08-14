use std::sync::Arc;

use web_sys::Element;

use crate::error::{FireEventError, QueryError};

pub type GetElementErrorFn = dyn Fn(Option<String>, Element) -> QueryError + Send + Sync;

pub type EventWrapperFn =
    dyn Fn(&dyn Fn() -> Result<bool, FireEventError>) -> Result<bool, FireEventError> + Send + Sync;

#[derive(Clone)]
pub struct Config {
    pub test_id_attribute: String,
    pub event_wrapper: Arc<EventWrapperFn>,
    // TODO
    /// Default value for the `hidden` option in `by_role` queries.
    pub default_hidden: bool,
    /// Default value for the `ignore` option in `by_text` queries.
    pub default_ignore: String,
    /// Flag to show the full error stack traces for async errors.
    pub show_original_stack_trace: bool,
    /// Throw errors with suggestions for better queries. Opt in so off by default.
    pub throw_suggestions: bool,
    // Called when `get_by` queries fail.
    pub get_element_error: Arc<GetElementErrorFn>,
}

impl Config {
    pub fn update(&mut self, other: PartialConfig) {
        if let Some(test_id_attribute) = other.test_id_attribute {
            self.test_id_attribute = test_id_attribute;
        }
        if let Some(event_wrapper) = other.event_wrapper {
            self.event_wrapper = event_wrapper;
        }
        if let Some(default_hidden) = other.default_hidden {
            self.default_hidden = default_hidden;
        }
        if let Some(default_ignore) = other.default_ignore {
            self.default_ignore = default_ignore;
        }
        if let Some(show_original_stack_trace) = other.show_original_stack_trace {
            self.show_original_stack_trace = show_original_stack_trace;
        }
        if let Some(throw_suggestions) = other.throw_suggestions {
            self.throw_suggestions = throw_suggestions;
        }
        if let Some(get_element_error) = other.get_element_error {
            self.get_element_error = get_element_error;
        }
    }
}

#[derive(Clone, Default)]
pub struct PartialConfig {
    pub test_id_attribute: Option<String>,
    pub event_wrapper: Option<Arc<EventWrapperFn>>,
    // TODO
    /// Default value for the `hidden` option in `by_role` queries.
    pub default_hidden: Option<bool>,
    /// Default value for the `ignore` option in `by_text` queries.
    pub default_ignore: Option<String>,
    /// Flag to show the full error stack traces for async errors.
    pub show_original_stack_trace: Option<bool>,
    /// Throw errors with suggestions for better queries. Opt in so off by default.
    pub throw_suggestions: Option<bool>,
    // Called when `get_by` queries fail.
    pub get_element_error: Option<Arc<GetElementErrorFn>>,
}

impl PartialConfig {
    pub fn test_id_attribute(mut self, value: String) -> Self {
        self.test_id_attribute = Some(value);
        self
    }

    pub fn event_wrapper(mut self, value: Arc<EventWrapperFn>) -> Self {
        self.event_wrapper = Some(value);
        self
    }

    pub fn default_hidden(mut self, value: bool) -> Self {
        self.default_hidden = Some(value);
        self
    }

    pub fn default_ignore(mut self, value: String) -> Self {
        self.default_ignore = Some(value);
        self
    }

    pub fn show_original_stack_trace(mut self, value: bool) -> Self {
        self.show_original_stack_trace = Some(value);
        self
    }

    pub fn throw_suggestions(mut self, value: bool) -> Self {
        self.throw_suggestions = Some(value);
        self
    }

    pub fn get_element_error(mut self, value: Arc<GetElementErrorFn>) -> Self {
        self.get_element_error = Some(value);
        self
    }
}

impl From<&Config> for PartialConfig {
    fn from(value: &Config) -> Self {
        Self {
            test_id_attribute: Some(value.test_id_attribute.clone()),
            event_wrapper: Some(value.event_wrapper.clone()),
            default_hidden: Some(value.default_hidden),
            default_ignore: Some(value.default_ignore.clone()),
            show_original_stack_trace: Some(value.show_original_stack_trace),
            throw_suggestions: Some(value.throw_suggestions),
            get_element_error: Some(value.get_element_error.clone()),
        }
    }
}

pub type ConfigFn = dyn Fn(&Config) -> PartialConfig;

pub enum ConfigFnOrPartial {
    Fn(Box<ConfigFn>),
    Partial(PartialConfig),
}
