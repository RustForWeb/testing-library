mod error;
mod types;

use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};

pub use error::PrettyFormatError;
pub use types::{Config, Plugin, PrettyFormatOptions, Printer, Refs};

use types::{Colors, Plugins};
use web_sys::js_sys::{BigInt, Number, Object};

fn print_number(val: &Number) -> String {
    if Object::is(val, &JsValue::from_f64(-0.0)) {
        "-0".into()
    } else {
        val.to_string(10)
            .expect("Number should be formatted as string.")
            .into()
    }
}

fn print_big_int(val: &BigInt) -> String {
    format!(
        "{}n",
        String::from(
            val.to_string(10)
                .expect("Number should be formatted as string.")
        )
    )
}

pub fn print_basic_value(
    val: &JsValue,
    _print_function_name: bool,
    _escape_regex: bool,
    escape_string: bool,
) -> Option<String> {
    if *val == JsValue::TRUE {
        return Some("true".into());
    }
    if *val == JsValue::FALSE {
        return Some("false".into());
    }
    if val.is_undefined() {
        return Some("undefined".into());
    }
    if val.is_null() {
        return Some("null".into());
    }

    let type_of = val.js_typeof();

    if type_of == "number" {
        return Some(print_number(val.unchecked_ref::<Number>()));
    }
    if type_of == "bigint" {
        return Some(print_big_int(val.unchecked_ref::<BigInt>()));
    }
    if type_of == "string" {
        if escape_string {
            return Some(
                val.as_string()
                    .expect("Value should be a string.")
                    .replace('"', "\\\"")
                    .replace('\\', "\\\\"),
            );
        }
        return Some(format!(
            "\"{}\"",
            val.as_string().expect("Value should be a string.")
        ));
    }

    todo!("print basic value {:?}", val)
}

pub fn print_complex_value(
    val: &JsValue,
    _config: &Config,
    _indentation: String,
    _depth: usize,
    _refs: Refs,
    _has_called_to_json: Option<bool>,
) -> String {
    todo!("print complex value {:?}", val)
}

fn print_plugin(
    plugin: Rc<dyn Plugin>,
    val: &JsValue,
    config: &Config,
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
    val: &JsValue,
    config: &Config,
    indentation: String,
    depth: usize,
    refs: Refs,
    has_called_to_json: Option<bool>,
) -> String {
    if let Some(plugin) = find_plugin(&config.plugins, val) {
        return print_plugin(plugin, val, config, indentation, depth, refs);
    }

    if let Some(basic_result) = print_basic_value(
        val,
        config.print_function_name,
        config.escape_regex,
        config.escape_string,
    ) {
        return basic_result;
    }

    print_complex_value(val, config, indentation, depth, refs, has_called_to_json)
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
    " ".repeat(indent)
}

pub fn format(val: &JsValue, options: PrettyFormatOptions) -> Result<String, PrettyFormatError> {
    validate_options(&options)?;

    if let Some(plugins) = &options.plugins {
        if let Some(plugin) = find_plugin(plugins, val) {
            return Ok(print_plugin(
                plugin,
                val,
                &get_config(options),
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
            &get_config(options),
            "".into(),
            0,
            vec![],
            None,
        ))
    }
}
