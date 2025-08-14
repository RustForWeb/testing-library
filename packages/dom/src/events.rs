use paste::paste;
use wasm_bindgen::JsValue;
use web_sys::{
    AnimationEvent, AnimationEventInit, ClipboardEvent, ClipboardEventInit, CompositionEvent,
    CompositionEventInit, DragEvent, DragEventInit, Event, EventInit, EventTarget, FocusEvent,
    FocusEventInit, InputEvent, InputEventInit, KeyboardEvent, KeyboardEventInit, MouseEvent,
    MouseEventInit, PageTransitionEvent, PageTransitionEventInit, PointerEvent, PointerEventInit,
    PopStateEvent, PopStateEventInit, ProgressEvent, ProgressEventInit, TouchEvent, TouchEventInit,
    TransitionEvent, TransitionEventInit, UiEvent, UiEventInit, WheelEvent, WheelEventInit,
};

use crate::{
    error::{CreateEventError, CreateOrFireEventError, FireEventError},
    get_config,
};

pub fn fire_event<E: EventType>(node: &EventTarget, event: &E) -> Result<bool, FireEventError> {
    (get_config().event_wrapper)(&|| {
        node.dispatch_event(event.deref_event())
            .map_err(FireEventError::JsError)
    })
}

pub type DefaultInitFn<E> = dyn Fn(&<E as EventType>::Init);

pub struct CreateEventOptions<'a, E: EventType> {
    default_init: Option<&'a DefaultInitFn<E>>,
}

impl<'a, E: EventType> CreateEventOptions<'a, E> {
    fn default_init(mut self, value: &'a DefaultInitFn<E>) -> Self {
        self.default_init = Some(value);
        self
    }
}

impl<'a, E: EventType> Default for CreateEventOptions<'a, E> {
    fn default() -> Self {
        Self {
            default_init: Default::default(),
        }
    }
}

pub fn create_event<E: EventType>(
    event_name: &str,
    _node: &EventTarget,
    init: Option<E::Init>,
    options: CreateEventOptions<E>,
) -> Result<E, CreateEventError> {
    let event_init = init.unwrap_or_default();

    if let Some(default_init) = options.default_init {
        default_init(&event_init);
    }

    E::new(event_name, &event_init).map_err(CreateEventError::JsError)
}

pub struct CreateEvent;

pub struct FireEvent;

macro_rules! generate_events {
    ($( ( $key:ident, $event_name:literal, $event_type:ty, { $( $init_key:ident : $init_value:literal ),* } ), )*) => {
        paste! {
            $(
                fn [<$key default_init>](init: &[<$event_type Init>]) {
                    $(
                        if init.[<get_ $init_key>]().is_none() {
                            init.[<set_ $init_key>]($init_value);
                        }
                    )*
                }
            )*

            impl CreateEvent {
                $(
                    pub fn $key(node: &EventTarget) -> Result<$event_type, CreateEventError> {
                        create_event($event_name, node, None, CreateEventOptions::default().default_init(&[<$key default_init>]))
                    }

                    pub fn [<$key _with_init>](node: &EventTarget, init: [<$event_type Init>]) -> Result<$event_type, CreateEventError> {
                        create_event($event_name, node, Some(init), CreateEventOptions::default().default_init(&[<$key default_init>]))
                    }
                )*
            }

            impl FireEvent {
                $(
                    pub fn $key(node: &EventTarget) -> Result<bool, CreateOrFireEventError> {
                        Ok(fire_event(node, &CreateEvent::$key(node)?)?)
                    }

                    pub fn [<$key _with_init>](node: &EventTarget, init: [<$event_type Init>]) -> Result<bool, CreateOrFireEventError> {
                        Ok(fire_event(node, &CreateEvent::[<$key _with_init>](node, init)?)?)
                    }
                )*
            }
        }
    };
}

generate_events!(
    // Clipboard Events
    (copy, "copy", ClipboardEvent, {bubbles: true, cancelable: true, composed: true}),
    (cut, "cut", ClipboardEvent, {bubbles: true, cancelable: true, composed: true}),
    (paste, "paste", ClipboardEvent, {bubbles: true, cancelable: true, composed: true}),
    // Composition Events
    (composition_end, "compositionend", CompositionEvent, {bubbles: true, cancelable: true, composed: true}),
    (composition_start, "compositionstart", CompositionEvent, {bubbles: true, cancelable: true, composed: true}),
    (composition_update, "compositionupdate", CompositionEvent, {bubbles: true, cancelable: true, composed: true}),
    // Keyboard Events
    (key_down, "keydown", KeyboardEvent, {bubbles: true, cancelable: true, char_code: 0, composed: true}),
    (key_press, "keypress", KeyboardEvent, {bubbles: true, cancelable: true, char_code: 0, composed: true}),
    (key_up, "keyup", KeyboardEvent, {bubbles: true, cancelable: true, char_code: 0, composed: true}),
    // Focus Events
    (focus, "focus", FocusEvent, {bubbles: false, cancelable: false, composed: true}),
    (blur, "blur", FocusEvent, {bubbles: false, cancelable: false, composed: true}),
    (focus_in, "focusin", FocusEvent, {bubbles: true, cancelable: false, composed: true}),
    (focus_out, "focusout", FocusEvent, {bubbles: true, cancelable: false, composed: true}),
    // Form Events
    (change, "change", Event, {bubbles: true, cancelable: false}),
    (input, "input", InputEvent, {bubbles: true, cancelable: false, composed: true}),
    (invalid, "invalid", Event, {bubbles: false, cancelable: true}),
    (submit, "submit", Event, {bubbles: true, cancelable: true}),
    (reset, "reset", Event, {bubbles: true, cancelable: true}),
    // Mouse Events
    (click, "click", MouseEvent, {bubbles: true, cancelable: true, button: 0, composed: true}),
    (context_menu, "contextmenu", MouseEvent, {bubbles: true, cancelable: true, composed: true}),
    (dbl_click, "dblclick", MouseEvent, {bubbles: true, cancelable: true, composed: true}),
    (drag, "drag", DragEvent, {bubbles: true, cancelable: true, composed: true}),
    (drag_end, "dragend", DragEvent, {bubbles: true, cancelable: false, composed: true}),
    (drag_enter, "dragenter", DragEvent, {bubbles: true, cancelable: true, composed: true}),
    (drag_exit, "dragexit", DragEvent, {bubbles: true, cancelable: false, composed: true}),
    (drag_leave, "dragleave", DragEvent, {bubbles: true, cancelable: false, composed: true}),
    (drag_over, "dragover", DragEvent, {bubbles: true, cancelable: true, composed: true}),
    (drag_start, "dragstart", DragEvent, {bubbles: true, cancelable: true, composed: true}),
    (drop, "drop", DragEvent, {bubbles: true, cancelable: true, composed: true}),
    (mouse_down, "mousedown", MouseEvent, {bubbles: true, cancelable: true, composed: true}),
    (mouse_enter, "mouseenter", MouseEvent, {bubbles: false, cancelable: false, composed: true}),
    (mouse_leave, "mouseleave", MouseEvent, {bubbles: false, cancelable: false, composed: true}),
    (mouse_move, "mousemove", MouseEvent, {bubbles: true, cancelable: true, composed: true}),
    (mouse_out, "mouseout", MouseEvent, {bubbles: true, cancelable: true, composed: true}),
    (mouse_over, "mouseover", MouseEvent, {bubbles: true, cancelable: true, composed: true}),
    (mouse_up, "mouseup", MouseEvent, {bubbles: true, cancelable: true, composed: true}),
    // Selection Events
    (select, "select", Event, {bubbles: true, cancelable: false}),
    // Touch Events
    (touch_cancel, "touchcancel", TouchEvent, {bubbles: true, cancelable: false, composed: true}),
    (touch_end, "touchend", TouchEvent, {bubbles: true, cancelable: true, composed: true}),
    (touch_move, "touchmove", TouchEvent, {bubbles: true, cancelable: true, composed: true}),
    (touch_start, "touchstart", TouchEvent, {bubbles: true, cancelable: true, composed: true}),
    // UI Events
    (resize, "resize", UiEvent, {bubbles: false, cancelable: false}),
    (scroll, "scroll", UiEvent, {bubbles: false, cancelable: false}),
    // Wheel Events
    (wheel, "wheel", WheelEvent, {bubbles: true, cancelable: true, composed: true}),
    // Media Events
    (abort, "abort", Event, {bubbles: false, cancelable: false}),
    (can_play, "canplay", Event, {bubbles: false, cancelable: false}),
    (can_play_through, "canplaythrough", Event, {bubbles: false, cancelable: false}),
    (duration_change, "durationchange", Event, {bubbles: false, cancelable: false}),
    (emptied, "emptied", Event, {bubbles: false, cancelable: false}),
    (encrypted, "encrypted", Event, {bubbles: false, cancelable: false}),
    (ended, "ended", Event, {bubbles: false, cancelable: false}),
    (loaded_data, "loadeddata", Event, {bubbles: false, cancelable: false}),
    (loaded_metadata, "loadedmetadata", Event, {bubbles: false, cancelable: false}),
    (load_start, "loadstart", ProgressEvent, {bubbles: false, cancelable: false}),
    (pause, "pause", Event, {bubbles: false, cancelable: false}),
    (play, "play", Event, {bubbles: false, cancelable: false}),
    (playing, "playing", Event, {bubbles: false, cancelable: false}),
    (progress, "progress", ProgressEvent, {bubbles: false, cancelable: false}),
    (rate_change, "ratechange", Event, {bubbles: false, cancelable: false}),
    (seeked, "seeked", Event, {bubbles: false, cancelable: false}),
    (seeking, "seeking", Event, {bubbles: false, cancelable: false}),
    (stalled, "stalled", Event, {bubbles: false, cancelable: false}),
    (suspend, "suspend", Event, {bubbles: false, cancelable: false}),
    (time_update, "timeupdate", Event, {bubbles: false, cancelable: false}),
    (volume_change, "volumechange", Event, {bubbles: false, cancelable: false}),
    (waiting, "waiting", Event, {bubbles: false, cancelable: false}),
    // Events
    // TODO: Load events can be UIEvent or Event depending on what generated them.
    // This is where this abstraction breaks down.
    // But the common targets are <img />, <script /> and window.
    // Neither of these targets receive a UIEvent.
    (load, "load", Event, {bubbles: false, cancelable: false}),
    (error, "error", Event, {bubbles: false, cancelable: false}),
    // Animation Events
    (animation_start, "animationstart", AnimationEvent, {bubbles: true, cancelable: false}),
    (animation_end, "animationend", AnimationEvent, {bubbles: true, cancelable: false}),
    (animation_iteration, "animationiteration", AnimationEvent, {bubbles: true, cancelable: false}),
    // Transition Events
    (transition_cancel, "transitioncancel", TransitionEvent, {bubbles: true, cancelable: false}),
    (transition_end, "transitionend", TransitionEvent, {bubbles: true, cancelable: true}),
    (transition_run, "transitionrun", TransitionEvent, {bubbles: true, cancelable: false}),
    (transition_start, "transitionstart", TransitionEvent, {bubbles: true, cancelable: false}),
    // Pointer events
    (pointer_over, "pointerover", PointerEvent, {bubbles: true, cancelable: true, composed: true}),
    (pointer_enter, "pointerenter", PointerEvent, {bubbles: false, cancelable: false}),
    (pointer_down, "pointerdown", PointerEvent, {bubbles: true, cancelable: true, composed: true}),
    (pointer_move, "pointermove", PointerEvent, {bubbles: true, cancelable: true, composed: true}),
    (pointer_up, "pointerup", PointerEvent, {bubbles: true, cancelable: true, composed: true}),
    (pointer_cancel, "pointercancel", PointerEvent, {bubbles: true, cancelable: false, composed: true}),
    (pointer_out, "pointerout", PointerEvent, {bubbles: true, cancelable: true, composed: true}),
    (pointer_leave, "pointerleave", PointerEvent, {bubbles: false, cancelable: false}),
    (got_pointer_capture, "gotpointercapture", PointerEvent, {bubbles: true, cancelable: false, composed: true}),
    (lost_pointer_capture, "lostpointercapture", PointerEvent, {bubbles: true, cancelable: false, composed: true}),
    // History events
    (pop_state, "popstate", PopStateEvent, {bubbles: true, cancelable: false}),
    // Window events
    (offline, "offline", Event, {bubbles: false, cancelable: false}),
    (online, "online", Event, {bubbles: false, cancelable: false}),
    (page_hide, "pagehide", PageTransitionEvent, {bubbles: true, cancelable: true}),
    (page_show, "pageshow", PageTransitionEvent, {bubbles: true, cancelable: true}),
);

// Aliases
impl CreateEvent {
    pub fn double_click(node: &EventTarget) -> Result<MouseEvent, CreateEventError> {
        CreateEvent::dbl_click(node)
    }

    pub fn double_click_with_init(
        node: &EventTarget,
        init: MouseEventInit,
    ) -> Result<MouseEvent, CreateEventError> {
        CreateEvent::dbl_click_with_init(node, init)
    }
}

// Aliases
impl FireEvent {
    pub fn double_click(node: &EventTarget) -> Result<bool, CreateOrFireEventError> {
        FireEvent::dbl_click(node)
    }

    pub fn double_click_with_init(
        node: &EventTarget,
        init: MouseEventInit,
    ) -> Result<bool, CreateOrFireEventError> {
        FireEvent::dbl_click_with_init(node, init)
    }
}

pub trait EventType {
    type Init: Default;

    fn new(r#type: &str, init: &Self::Init) -> Result<Self, JsValue>
    where
        Self: std::marker::Sized;

    fn deref_event(&self) -> &Event;
}

macro_rules! generate_event_types {
    ($( ( $event_type:ty, $func:ident ), )*) => {
        paste! {
            $(
                impl EventType for $event_type {
                    type Init = [<$event_type Init>];

                    fn new(r#type: &str, init: &Self::Init) -> Result<Self, JsValue> {
                        Self::$func(r#type, init)
                    }

                    fn deref_event(&self) -> &Event {
                        self
                    }
                }
            )*
        }
    };
}

generate_event_types!(
    (AnimationEvent, new_with_event_init_dict),
    (ClipboardEvent, new_with_event_init_dict),
    (CompositionEvent, new_with_event_init_dict),
    (DragEvent, new_with_event_init_dict),
    (Event, new_with_event_init_dict),
    (FocusEvent, new_with_focus_event_init_dict),
    (InputEvent, new_with_event_init_dict),
    (KeyboardEvent, new_with_keyboard_event_init_dict),
    (MouseEvent, new_with_mouse_event_init_dict),
    (PageTransitionEvent, new_with_event_init_dict),
    (PointerEvent, new_with_event_init_dict),
    (PopStateEvent, new_with_event_init_dict),
    (ProgressEvent, new_with_event_init_dict),
    (TouchEvent, new_with_event_init_dict),
    (TransitionEvent, new_with_event_init_dict),
    (UiEvent, new_with_event_init_dict),
    (WheelEvent, new_with_event_init_dict),
);
