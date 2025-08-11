#![cfg(target_arch = "wasm32")]

mod helpers;

use std::sync::{Arc, LazyLock, Mutex, MutexGuard};

use mockall::automock;
use testing_library_dom::{ConfigFnOrPartial, FireEvent, PartialConfig, configure};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use self::helpers::test_utils::document;

wasm_bindgen_test_configure!(run_in_browser);

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

#[wasm_bindgen_test]
fn fire_event_calls_the_event_wrapper() {
    let lock = before_each();

    #[automock]
    trait EventWrapper {
        fn call(&self);
    }

    let mut mock = MockEventWrapper::new();
    mock.expect_call().times(1).return_const(());

    configure(ConfigFnOrPartial::Partial(
        PartialConfig::default().event_wrapper(Arc::new(move |cb| {
            mock.call();
            cb()
        })),
    ));

    let el = document()
        .create_element("div")
        .expect("Element should be created.");

    FireEvent::click(&el).expect("Event should be fired.");

    after_each(lock);
}
