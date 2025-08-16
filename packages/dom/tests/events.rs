// TODO: Enable `cfg`, disable `expect`.
// #![cfg(target_arch = "wasm32")]
#![allow(dead_code)]

mod helpers;

use mockall::automock;
use send_wrapper::SendWrapper;
use testing_library_dom::{CreateEventOptions, FireEvent, create_event, fire_event};
use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::{
    AddEventListenerOptions, Event, KeyboardEvent, KeyboardEventInit, MessageEvent,
    MessageEventInit, MouseEvent, PopStateEvent, PopStateEventInit,
    js_sys::{Object, Reflect},
    window,
};

use self::helpers::test_utils::document;

wasm_bindgen_test_configure!(run_in_browser);

#[automock]
trait Spy {
    fn call_event(&self, event: SendWrapper<Event>);

    fn call_keyboard_event(&self, event: SendWrapper<KeyboardEvent>);

    fn call_message_event(&self, event: SendWrapper<MessageEvent>);

    fn call_mouse_event(&self, event: SendWrapper<MouseEvent>);

    fn call_pop_state_event(&self, event: SendWrapper<PopStateEvent>);
}

// TODO: More tests.

#[wasm_bindgen_test]
fn fires_resize() {
    let window = document()
        .default_view()
        .expect("Document should have default view.");

    let mut mock = MockSpy::new();
    mock.expect_call_event().times(1).return_const(());

    let listener = Closure::<dyn Fn(Event)>::new(move |event| {
        mock.call_event(SendWrapper::new(event));
    });

    let options = AddEventListenerOptions::new();
    options.set_once(true);
    window
        .add_event_listener_with_callback_and_add_event_listener_options(
            "resize",
            listener.as_ref().unchecked_ref(),
            &options,
        )
        .expect("Event listener should be added.");

    FireEvent::resize(&window).expect("Event should be fired.");
}

// TODO: More tests.

#[wasm_bindgen_test]
fn fires_events_on_window() {
    let window = window().expect("Window should exist.");

    let mut mock = MockSpy::new();
    mock.expect_call_message_event().times(1).return_const(());

    let listener = Closure::<dyn Fn(MessageEvent)>::new(move |event| {
        mock.call_message_event(SendWrapper::new(event));
    });

    window
        .add_event_listener_with_callback("message", listener.as_ref().unchecked_ref())
        .expect("Event listener should be added.");

    let init = MessageEventInit::new();
    init.set_data(&"hello".into());
    let event = MessageEvent::new_with_event_init_dict("message", &init)
        .expect("Event should be constructed.");

    fire_event(&window, &event).expect("Event should be fired.");

    window
        .remove_event_listener_with_callback("message", listener.as_ref().unchecked_ref())
        .expect("Event listener should be removed.");
}

#[wasm_bindgen_test]
fn fires_history_popstate_event_on_window() {
    let window = window().expect("Window should exist.");

    let mut mock = MockSpy::new();
    mock.expect_call_pop_state_event().times(1).return_const(());

    let listener = Closure::<dyn Fn(PopStateEvent)>::new(move |event| {
        mock.call_pop_state_event(SendWrapper::new(event));
    });

    window
        .add_event_listener_with_callback("popstate", listener.as_ref().unchecked_ref())
        .expect("Event listener should be added.");

    let state = Object::new();
    Reflect::set(&state, &"page".into(), &1.into()).expect("Propety should be set.");

    let init = PopStateEventInit::new();
    init.set_state(&state);
    FireEvent::pop_state_with_init(&window, &init).expect("Event should be fired.");

    window
        .remove_event_listener_with_callback("popstate", listener.as_ref().unchecked_ref())
        .expect("Event listener should be removed.");
}

#[wasm_bindgen_test]
fn fires_shortcut_events_on_window() {
    let window = window().expect("Window should exist.");

    let mut mock = MockSpy::new();
    mock.expect_call_mouse_event().times(1).return_const(());

    let listener = Closure::<dyn Fn(MouseEvent)>::new(move |event| {
        mock.call_mouse_event(SendWrapper::new(event));
    });

    window
        .add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
        .expect("Event listener should be added.");

    FireEvent::click(&window).expect("Event should be fired.");

    window
        .remove_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
        .expect("Event listener should be removed.");
}

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
    FireEvent::key_down_with_init(&document, &init).expect("Event should be fired.");

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
