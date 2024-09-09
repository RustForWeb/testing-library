use std::rc::Rc;

use wasm_bindgen::JsValue;

#[derive(Clone, Debug, Default)]
pub struct Color {
    open: String,
    close: String,
}

impl Color {
    pub fn open(&self) -> String {
        self.open.clone()
    }

    pub fn close(&self) -> String {
        self.close.clone()
    }

    pub fn paint(&self, s: &str) -> String {
        format!("{}{}{}", self.open(), s, self.close())
    }
}

impl From<ansi_style::Color> for Color {
    fn from(value: ansi_style::Color) -> Self {
        Color {
            open: value.open(),
            close: value.close().into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Colors {
    pub comment: Color,
    pub content: Color,
    pub prop: Color,
    pub tag: Color,
    pub value: Color,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub comment: Color,
    pub content: Color,
    pub prop: Color,
    pub tag: Color,
    pub value: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            comment: ansi_style::Color::BlackBright.into(),
            content: Color {
                // Reset
                open: "\x1B[0m".into(),
                close: "\x1B[0m".into(),
            },
            prop: ansi_style::Color::Yellow.into(),
            tag: ansi_style::Color::Cyan.into(),
            value: ansi_style::Color::Green.into(),
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

pub type Printer = dyn Fn(&JsValue, &Config, String, usize, Refs, Option<bool>) -> String;

pub trait Plugin {
    fn test(&self, val: &JsValue) -> bool;

    fn serialize(
        &self,
        val: &JsValue,
        config: &Config,
        indentation: String,
        depth: usize,
        refs: Refs,
        printer: &Printer,
    ) -> String;
}

pub type Plugins = Vec<Rc<dyn Plugin>>;
