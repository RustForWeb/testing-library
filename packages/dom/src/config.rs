use std::sync::{Arc, LazyLock, Mutex};

use crate::{
    error::QueryError,
    pretty_dom::pretty_dom,
    types::{Config, ConfigFnOrPartial},
};

static CONFIG: LazyLock<Arc<Mutex<Config>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(Config {
        test_id_attribute: "data-testid".into(),
        default_hidden: false,
        default_ignore: "script, style".into(),
        show_original_stack_trace: false,
        throw_suggestions: false,
        get_element_error: Arc::new(|message, container| {
            let prettified_dom = pretty_dom(Some(container.into()), None);

            let default_ignore = {
                let config = CONFIG.lock().expect("Config mutex should be acquired.");
                config.default_ignore.clone()
            };

            QueryError::Element(
                [
                    message,
                    Some(format!(
                        "Ignored nodes: comments, {default_ignore}\n{prettified_dom}"
                    )),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join("\n\n"),
            )
        }),
    }))
});

pub fn configure(new_config: ConfigFnOrPartial) {
    let mut config = CONFIG.lock().expect("Config mutex should be acquired.");

    let new_config = match new_config {
        ConfigFnOrPartial::Fn(config_fn) => config_fn(&config),
        ConfigFnOrPartial::Partial(new_config) => new_config,
    };

    config.update(new_config);
}

pub fn get_config() -> Config {
    let config = CONFIG.lock().expect("Config mutex should be acquired.");

    (*config).clone()
}
