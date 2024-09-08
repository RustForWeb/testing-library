// TODO: remove
#![allow(dead_code, unused)]

mod error;
mod types;

use std::rc::Rc;

use wasm_bindgen::JsValue;

pub use error::PrettyFormatError;
pub use types::{Config, Plugin, PrettyFormatOptions, Printer, Refs};

use types::{Colors, Plugins};

pub fn print_basic_value(
    val: &JsValue,
    print_function_name: bool,
    escape_regex: bool,
    escape_string: bool,
) -> Option<String> {
    // TODO
    None
}

pub fn print_complex_value(
    val: &JsValue,
    config: Config,
    indentation: String,
    depth: usize,
    refs: Refs,
    has_called_to_json: Option<bool>,
) -> String {
    "".into()
}

fn print_plugin(
    plugin: Rc<dyn Plugin>,
    val: &JsValue,
    config: Config,
    indentation: String,
    depth: usize,
    refs: Refs,
) -> String {
    plugin.serialize(val, config, indentation, depth, refs, &printer)
}

fn find_plugin(plugins: &Plugins, val: &JsValue) -> Option<Rc<dyn Plugin>> {
    plugins.iter().find(|plugin| plugin.test(val)).cloned()
}

fn printer(
    val: JsValue,
    config: Config,
    indentation: String,
    depth: usize,
    refs: Refs,
    has_called_to_json: Option<bool>,
) -> String {
    "".into()
}

fn validate_options(options: &PrettyFormatOptions) -> Result<(), PrettyFormatError> {
    if options.min.is_some() && options.indent.is_some_and(|indent| indent != 0) {
        Err(PrettyFormatError::Configuration(
            "Options `min` and `indent` cannot be used togther.".into(),
        ))
    } else {
        Ok(())
    }
}

fn get_colors_highlight(options: &PrettyFormatOptions) -> Colors {
    let theme = options.theme.clone().unwrap_or_default();

    Colors {
        comment: theme.comment,
        content: theme.content,
        prop: theme.prop,
        tag: theme.tag,
        value: theme.value,
    }
}

fn get_colors_empty() -> Colors {
    Colors::default()
}

fn get_print_function_name(options: &PrettyFormatOptions) -> bool {
    options.print_function_name.unwrap_or(true)
}

fn get_escape_regex(options: &PrettyFormatOptions) -> bool {
    options.escape_regex.unwrap_or(false)
}

fn get_escape_string(options: &PrettyFormatOptions) -> bool {
    options.escape_string.unwrap_or(true)
}

fn get_config(options: PrettyFormatOptions) -> Config {
    Config {
        call_to_json: options.call_to_json.unwrap_or(true),
        compare_keys: options.compare_keys.clone(),
        colors: match options.highlight {
            Some(true) => get_colors_highlight(&options),
            _ => get_colors_empty(),
        },
        escape_regex: options.escape_regex.unwrap_or(false),
        escape_string: options.escape_string.unwrap_or(true),
        indent: match options.min {
            Some(true) => "".into(),
            _ => create_indent(options.indent.unwrap_or(2)),
        },
        max_depth: options.max_depth.unwrap_or(usize::MAX),
        max_width: options.max_width.unwrap_or(usize::MAX),
        min: options.min.unwrap_or(false),
        plugins: options.plugins.unwrap_or_default(),
        print_function_name: options.print_function_name.unwrap_or(true),
        spacing_inner: match options.min {
            Some(true) => " ",
            _ => "\n",
        }
        .into(),
        spacing_outer: match options.min {
            Some(true) => "",
            _ => "\n",
        }
        .into(),
    }
}

fn create_indent(indent: usize) -> String {
    " ".repeat(indent + 1)
}

pub fn format(val: &JsValue, options: PrettyFormatOptions) -> Result<String, PrettyFormatError> {
    validate_options(&options)?;

    if let Some(plugins) = &options.plugins {
        if let Some(plugin) = find_plugin(plugins, val) {
            return Ok(print_plugin(
                plugin,
                val,
                get_config(options),
                "".into(),
                0,
                vec![],
            ));
        }
    }

    let basic_result = print_basic_value(
        val,
        get_print_function_name(&options),
        get_escape_regex(&options),
        get_escape_string(&options),
    );
    if let Some(basic_result) = basic_result {
        Ok(basic_result)
    } else {
        Ok(print_complex_value(
            val,
            get_config(options),
            "".into(),
            0,
            vec![],
            None,
        ))
    }
}
