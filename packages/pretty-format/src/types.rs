use std::rc::Rc;

use ansi_style::{Style, StyleBuilder};

// TODO: consider replacing this with JsValue, so code can match the JS implementation
#[derive(Debug)]
pub enum PrettyFormatValue {
    Bool(bool),
}

impl From<bool> for PrettyFormatValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

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

pub type Refs = Vec<PrettyFormatValue>;

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
    // pub print_basic_prototype: Option<bool>,
    pub print_function_name: Option<bool>,
    pub theme: Option<Theme>,
    pub compare_keys: Option<CompareKeys>,
    pub plugins: Option<Plugins>,
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

pub type Printer = dyn Fn(PrettyFormatValue, Config, String, usize, Refs, Option<bool>) -> String;

pub trait Plugin {
    fn serialize(
        &self,
        val: PrettyFormatValue,
        config: Config,
        indentation: String,
        depth: usize,
        refs: Refs,
        printer: &Printer,
    ) -> String;

    fn test(&self, val: &PrettyFormatValue) -> bool;
}

pub type Plugins = Vec<Rc<dyn Plugin>>;
