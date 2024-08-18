use std::sync::{Arc, LazyLock, Mutex};

use testing_library_dom::{configure, get_config, ConfigFnOrPartial, PartialConfig};

static ORIGINAL_CONFIG: LazyLock<Arc<Mutex<PartialConfig>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PartialConfig::default())));

fn before_each() {
    configure(ConfigFnOrPartial::Fn(Box::new(|existing_config| {
        // Grab the existing configuration so we can restore it at the end of the test.
        let mut original_config = ORIGINAL_CONFIG
            .lock()
            .expect("Original config mutex should be acquired.");
        *original_config = PartialConfig::from(existing_config);

        // Don't change the existing config.
        PartialConfig::default()
    })));
}

fn after_each() {
    let original_config = ORIGINAL_CONFIG
        .lock()
        .expect("Original config mutex should be acquired.");

    configure(ConfigFnOrPartial::Partial((*original_config).clone()));
}

#[test]
fn get_config_returns_existing_configuration() {
    before_each();

    let config = get_config();
    assert_eq!("data-testid", config.test_id_attribute);

    after_each();
}

#[test]
fn configure_merges_a_delta_rather_than_replacing_the_whole_config() {
    before_each();

    let config = get_config();
    assert_eq!("data-testid", config.test_id_attribute);

    after_each();
}

#[test]
fn configure_overrides_existing_values() {
    before_each();

    configure(ConfigFnOrPartial::Partial(
        PartialConfig::default().test_id_attribute("new-id".into()),
    ));

    let config = get_config();
    assert_eq!("new-id", config.test_id_attribute);

    after_each();
}

#[test]
fn configure_passes_existing_config_out_to_config_function() {
    before_each();

    // Create a new config key based on the value of an existing one.
    configure(ConfigFnOrPartial::Fn(Box::new(|existing_config| {
        PartialConfig::default()
            .test_id_attribute(format!("{}-derived", existing_config.test_id_attribute))
    })));

    // The new value should be there, and existing values should be untouched.
    let config = get_config();
    assert_eq!("data-testid-derived", config.test_id_attribute);

    after_each();
}
