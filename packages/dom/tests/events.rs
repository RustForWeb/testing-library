// TODO: Enable `cfg`, disable `expect`.
// #![cfg(target_arch = "wasm32")]
#![allow(dead_code)]

mod helpers;

use mockall::automock;
use send_wrapper::SendWrapper;
use testing_library_dom::{CreateEventOptions, FireEvent, create_event, fire_event};
use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::{Event, KeyboardEvent, KeyboardEventInit};

use self::helpers::test_utils::document;

wasm_bindgen_test_configure!(run_in_browser);

#[automock]
trait Spy {
    fn call_event(&self, event: SendWrapper<Event>);

    fn call_keyboard_event(&self, event: SendWrapper<KeyboardEvent>);
}

// TODO: More tests.

#[wasm_bindgen_test]
fn fires_events_on_document() {
    let document = document();

    let mut mock = MockSpy::new();
    mock.expect_call_keyboard_event().times(1).return_const(());

    let listener = Closure::<dyn Fn(KeyboardEvent)>::new(move |event| {
        mock.call_keyboard_event(SendWrapper::new(event));
    });

    document
        .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
        .expect("Event listener should be added.");

    let init = KeyboardEventInit::new();
    init.set_key("Escape");
    FireEvent::key_down_with_init(&document, init).expect("Event should be fired.");

    document
        .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
        .expect("Event listener should be removed.");
}

#[wasm_bindgen_test]
fn can_create_generic_events() {
    let el = document()
        .create_element("div")
        .expect("Element should be created.");
    let event_name = "my-custom-event";

    let event = create_event::<Event>(event_name, &el, None, CreateEventOptions::default())
        .expect("Event should be created.");
    let wrapped_event = SendWrapper::new(event.clone());

    let mut mock = MockSpy::new();
    mock.expect_call_event()
        .withf(move |e| **e == *wrapped_event)
        .times(1)
        .return_const(());

    let listener = Closure::<dyn Fn(Event)>::new(move |event| {
        mock.call_event(SendWrapper::new(event));
    });
    el.add_event_listener_with_callback(event_name, listener.as_ref().unchecked_ref())
        .expect("Event listener should be added.");

    fire_event(&el, &event).expect("Event should be fired");
}
