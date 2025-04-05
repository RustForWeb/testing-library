use std::sync::{Arc, LazyLock, Mutex, MutexGuard};

use testing_library_dom::{ConfigFnOrPartial, PartialConfig, configure, get_config};

static CONFIG_LOCK: Mutex<()> = Mutex::new(());
static ORIGINAL_CONFIG: LazyLock<Arc<Mutex<PartialConfig>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PartialConfig::default())));

fn before_each() -> MutexGuard<'static, ()> {
    // Ensure only one test modifies the config at the same time.
    let lock = CONFIG_LOCK
        .lock()
        .expect("Config mutex should be acquired.");

    configure(ConfigFnOrPartial::Fn(Box::new(|existing_config| {
        // Grab the existing configuration so we can restore it at the end of the test.
        let mut original_config = ORIGINAL_CONFIG
            .lock()
            .expect("Original config mutex should be acquired.");
        *original_config = PartialConfig::from(existing_config);

        // Don't change the existing config.
        PartialConfig::default()
    })));

    lock
}

fn after_each(lock: MutexGuard<'_, ()>) {
    let original_config = ORIGINAL_CONFIG
        .lock()
        .expect("Original config mutex should be acquired.");

    configure(ConfigFnOrPartial::Partial((*original_config).clone()));

    drop(lock);
}

#[test]
fn get_config_returns_existing_configuration() {
    let lock = before_each();

    let config = get_config();
    assert_eq!("data-testid", config.test_id_attribute);

    after_each(lock);
}

#[test]
fn configure_merges_a_delta_rather_than_replacing_the_whole_config() {
    let lock = before_each();

    let config = get_config();
    assert_eq!("data-testid", config.test_id_attribute);

    after_each(lock);
}

#[test]
fn configure_overrides_existing_values() {
    let lock = before_each();

    configure(ConfigFnOrPartial::Partial(
        PartialConfig::default().test_id_attribute("new-id".into()),
    ));

    let config = get_config();
    assert_eq!("new-id", config.test_id_attribute);

    after_each(lock);
}

#[test]
fn configure_passes_existing_config_out_to_config_function() {
    let lock = before_each();

    // Create a new config key based on the value of an existing one.
    configure(ConfigFnOrPartial::Fn(Box::new(|existing_config| {
        PartialConfig::default()
            .test_id_attribute(format!("{}-derived", existing_config.test_id_attribute))
    })));

    // The new value should be there, and existing values should be untouched.
    let config = get_config();
    assert_eq!("data-testid-derived", config.test_id_attribute);

    after_each(lock);
}
