use std::sync::Arc;

use web_sys::Element;

use crate::error::QueryError;

pub type GetElementErrorFn = dyn Fn(Option<String>, Element) -> QueryError + Send + Sync;

#[derive(Clone)]
pub struct Config {
    pub test_id_attribute: String,
    // TODO
    pub computed_style_supports_pseudo_elements: bool,
    pub default_hidden: bool,
    pub default_ignore: String,
    pub show_original_stack_trace: bool,
    pub throw_suggestions: bool,
    pub get_element_error: Arc<GetElementErrorFn>,
}

impl Config {
    pub fn update(&mut self, other: PartialConfig) {
        if let Some(test_id_attribute) = other.test_id_attribute {
            self.test_id_attribute = test_id_attribute;
        }
        if let Some(computed_style_supports_pseudo_elements) =
            other.computed_style_supports_pseudo_elements
        {
            self.computed_style_supports_pseudo_elements = computed_style_supports_pseudo_elements;
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
    // TODO
    pub computed_style_supports_pseudo_elements: Option<bool>,
    pub default_hidden: Option<bool>,
    pub default_ignore: Option<String>,
    pub show_original_stack_trace: Option<bool>,
    pub throw_suggestions: Option<bool>,
    pub get_element_error: Option<Arc<GetElementErrorFn>>,
}

impl PartialConfig {
    pub fn test_id_attribute(mut self, value: String) -> Self {
        self.test_id_attribute = Some(value);
        self
    }

    pub fn computed_style_supports_pseudo_elements(mut self, value: bool) -> Self {
        self.computed_style_supports_pseudo_elements = Some(value);
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
            computed_style_supports_pseudo_elements: Some(
                value.computed_style_supports_pseudo_elements,
            ),
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
