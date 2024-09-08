use std::rc::Rc;

use ansi_style::{Style, StyleBuilder};
use wasm_bindgen::JsValue;

#[derive(Debug, Default)]
pub struct Colors {
    pub comment: Style,
    pub content: Style,
    pub prop: Style,
    pub tag: Style,
    pub value: Style,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub comment: Style,
    pub content: Style,
    pub prop: Style,
    pub tag: Style,
    pub value: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            comment: StyleBuilder::new().black_bright().build(),
            content: StyleBuilder::new().build(),
            prop: StyleBuilder::new().yellow().build(),
            tag: StyleBuilder::new().cyan().build(),
            value: StyleBuilder::new().green().build(),
        }
    }
}

pub type Refs = Vec<JsValue>;

pub type CompareKeys = Rc<dyn Fn(String, String) -> usize>;

#[derive(Default)]
pub struct PrettyFormatOptions {
    pub call_to_json: Option<bool>,
    pub escape_regex: Option<bool>,
    pub escape_string: Option<bool>,
    pub highlight: Option<bool>,
    pub indent: Option<usize>,
    pub max_depth: Option<usize>,
    pub max_width: Option<usize>,
    pub min: Option<bool>,
    pub print_basic_prototype: Option<bool>,
    pub print_function_name: Option<bool>,
    pub theme: Option<Theme>,
    pub compare_keys: Option<CompareKeys>,
    pub plugins: Option<Plugins>,
}

impl PrettyFormatOptions {
    pub fn call_to_json(mut self, value: bool) -> Self {
        self.call_to_json = Some(value);
        self
    }

    pub fn escape_regex(mut self, value: bool) -> Self {
        self.escape_regex = Some(value);
        self
    }

    pub fn escape_string(mut self, value: bool) -> Self {
        self.escape_string = Some(value);
        self
    }

    pub fn highlight(mut self, value: bool) -> Self {
        self.highlight = Some(value);
        self
    }

    pub fn indent(mut self, value: usize) -> Self {
        self.indent = Some(value);
        self
    }

    pub fn max_depth(mut self, value: usize) -> Self {
        self.max_depth = Some(value);
        self
    }

    pub fn max_width(mut self, value: usize) -> Self {
        self.max_width = Some(value);
        self
    }

    pub fn min(mut self, value: bool) -> Self {
        self.min = Some(value);
        self
    }

    pub fn print_basic_prototype(mut self, value: bool) -> Self {
        self.print_basic_prototype = Some(value);
        self
    }

    pub fn print_function_name(mut self, value: bool) -> Self {
        self.print_function_name = Some(value);
        self
    }

    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = Some(value);
        self
    }

    pub fn compare_keys(mut self, value: CompareKeys) -> Self {
        self.compare_keys = Some(value);
        self
    }

    pub fn plugins(mut self, value: Plugins) -> Self {
        self.plugins = Some(value);
        self
    }
}

pub struct Config {
    pub call_to_json: bool,
    pub compare_keys: Option<CompareKeys>,
    pub colors: Colors,
    pub escape_regex: bool,
    pub escape_string: bool,
    pub indent: String,
    pub max_depth: usize,
    pub max_width: usize,
    pub min: bool,
    pub plugins: Plugins,
    // pub print_basic_prototype: bool,
    pub print_function_name: bool,
    pub spacing_inner: String,
    pub spacing_outer: String,
}

pub type Printer = dyn Fn(JsValue, Config, String, usize, Refs, Option<bool>) -> String;

pub trait Plugin {
    fn test(&self, val: &JsValue) -> bool;

    fn serialize(
        &self,
        val: &JsValue,
        config: Config,
        indentation: String,
        depth: usize,
        refs: Refs,
        printer: &Printer,
    ) -> String;
}

pub type Plugins = Vec<Rc<dyn Plugin>>;
