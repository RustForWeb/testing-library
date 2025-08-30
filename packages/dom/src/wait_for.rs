use std::{cell::RefCell, pin::Pin, rc::Rc};

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::window;

use crate::{
    DocumentOrElement, error::WaitForError, get_config, helpers::get_document,
    types::WaitForOptions,
};

pub async fn wait_for<T, E: 'static>(
    // TODO: Remove if not using async.
    // callback: impl Fn() -> Pin<Box<dyn Future<Output = Result<T, E>>>>,
    callback: impl Fn() -> Result<T, E>,
    options: WaitForOptions,
) -> Result<T, WaitForError<E>> {
    let config = get_config();
    let container: DocumentOrElement = options
        .container
        .map(Into::into)
        .unwrap_or_else(|| get_document().into());
    let timeout = options.timeout.unwrap_or(config.async_util_timeout);
    let interval = 50;

    let window = window().expect("Window should exist.");
    let last_error: Rc<RefCell<Option<E>>> = Rc::new(RefCell::new(None));

    let check_callback = {
        let last_error = last_error.clone();

        move || {
            match callback() {
                Ok(result) => {
                    //TODO
                }
                Err(error) => {
                    last_error.replace(Some(error));
                }
            }
        }
    };

    let handle_timeout = Closure::<dyn Fn()>::new(move || {
        let error = last_error
            .take()
            .map(Into::into)
            .unwrap_or(WaitForError::TimedOut);

        // TODO
    });

    let overal_timeout_id = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            handle_timeout.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("Timeout should be started.");

    // let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(handler, timeout)

    let on_done = || {
        window.clear_timeout_with_handle(overal_timeout_id);
        // TODO
    };

    Err(WaitForError::TimedOut)
}
