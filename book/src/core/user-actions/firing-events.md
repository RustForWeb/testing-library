# Firing Events

<!-- > **Note**
>
> Most projects have a few use cases for `fire_event`, but the majority of the time you should probably use `testing-library-user-event`. -->

## `fire_event`

```rust,ignore
use testing_library_dom::{EventType, FireEventError};
use web_sys::EventTarget;

fn fire_event<E: EventType>(node: &EventTarget, event: &E) -> Result<bool, FireEventError>;
```

Fire DOM events.

```rust,ignore
// HTML: <button>Submit</button>

use testing_library_dom::{get_by_text, fire_event};
use web_sys::MouseEvent;

let event = MouseEvent::new("click");
event.set_bubbles(true);
event.set_cancelable(true);

fire_event(
    get_by_text(&container, "Submit")
        .expect("Get should succeed.")
        .expect("Get should return an element."),
    &event,
).expect("Event should be fired.");
```

## `FireEvent::[<event_name>]`

```rust,ignore
fn [<event_name>](node: &EventTarget, event_properties: &[<EventInit>]) -> Result<bool, CreateOrFireEventError>;
```

Convenience methods for firing DOM events. Check out `src/events.rs` for a full list as well as default event proprties.

<!-- TODO: target, data transfer -->

### Keyboard events

There are three event types related to keyboard input - `keyPress`, `keyDown`, and `keyUp`. When firing these you need to reference an element in the DOM and the key you want to fire.

```rust,ignore
use testing_library_dom::FireEvent;
use web_sys::KeyboardEventInit;

let init = KeyboardEventInit::new();
init.set_key("Enter");
init.set_code("Enter");
init.set_char_code(13);
FireEvent::key_down(&dom_node, &init).expect("Event should be fired.");

let init = KeyboardEventInit::new();
init.set_key("A");
init.set_code("KeyA");
FireEvent::key_down(&dom_node, &init).expect("Event should be fired.");
```

You can find out which key code to use at https://www.toptal.com/developers/keycode.

## `CreateEvent::[<event_name>]`

```rust,ignore
fn [<event_name>](node: &EventTarget, event_properties: &[<EventInit>]) -> Result<[<Event>], CreateOrFireEventError>;
```

Convenience methods for creating DOM events that can then be fired by `fire_event`, allowing you to have a reference to the event created: this might be useful if you need to access event properties that cannot be initiated programmatically (such as [`time_stamp`](https://docs.rs/web-sys/latest/web_sys/struct.Event.html#method.time_stamp)).

```rust,ignore
use testing_library_dom::{CreateEvent, fire_event};
use web_sys::MouseEventInit;

let init = MouseEventInit::new();
init.set_button(2);
let my_event = CreateEvent::click(&node, &init).expect("Event should be created.");

fire_event(&node, &my_event).expect("Event should be fired.");

// `my_event.time_stamp()` can be accessed just like any other properties from `my_event`.
// Note: The access to the events created by `create_event` is based on the native event API.
// Therefore, native properties of HTML Event object (e.g. `timeStamp`, `cancelable`, `type`) should be set using `Object.defineProperty`.
// For more info see: https://developer.mozilla.org/en-US/docs/Web/API/Event.
```

## `create_event`

```rust,ignore
use testing_library_dom::{CreateEventError, EventType};
use web_sys::EventTarget;

fn create_event<E: EventType>(
    event_name: &str,
    node: &EventTarget,
    init: Option<&E::Init>,
    options: CreateEventOptions<E>,
) -> Result<E, CreateEventError>;

struct CreateEventOptions<'a, E: EventType> {
    default_init: Option<&'a DefaultInitFn<E>>,
}

type DefaultInitFn<E> = dyn Fn(&<E as EventType>::Init);
```

Create DOM events.

```rust,ignore
use testing_library_dom::{CreateEventOptions, create_event};
use web_sys::{InputEvent, InputEventInit};

let init = InputEventInit::new();
init.set_data(Some("a"));
let event = create_event::<InputEvent>(
    "input",
    &input,
    Some(&init),
    CreateEventOptions::default()
).expect("Event should be created.");
```
