use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Promise, window};

use crate::{
    DocumentOrElement, error::WaitForError, get_config, helpers::get_document,
    types::WaitForOptions,
};

pub async fn wait_for<T, E: 'static>(
    callback: impl Fn() -> Result<T, E>,
    options: WaitForOptions,
) -> Result<T, WaitForError<E>> {
    // TODO: Consider implementing this function using Rust async instead of promises.

    // JsFuture::from(wait_for_promise(callback, options)).await.map(|result| ).map_err(|error|)
    todo!()
}

fn wait_for_promise<T: Into<JsValue> + 'static, E: Into<JsValue> + 'static>(
    callback: impl Fn() -> Result<T, E> + 'static,
    options: WaitForOptions,
) -> Promise {
    let config = get_config();
    let container: DocumentOrElement = options
        .container
        .map(Into::into)
        .unwrap_or_else(|| get_document().into());
    let timeout = options.timeout.unwrap_or(config.async_util_timeout);
    let interval = 50;

    let callback = Rc::new(callback);

    Promise::new(&mut move |resolve, reject| {
        let window = window().expect("Window should exist.");
        let last_error: Rc<RefCell<Option<E>>> = Rc::new(RefCell::new(None));
        let overal_timeout_id: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
        let interval_id: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

        let handle_timeout = Closure::<dyn Fn()>::new({
            let last_error = last_error.clone();

            move || {
                let error = last_error
                    .take()
                    .map(WaitForError::Error)
                    .unwrap_or(WaitForError::TimedOut);

                // TODO
            }
        });

        let on_done = {
            let window = window.clone();
            let overal_timeout_id = overal_timeout_id.clone();
            let interval_id = interval_id.clone();

            move |result: Result<T, E>| {
                if let Some(overal_timeout_id) = overal_timeout_id.take() {
                    window.clear_timeout_with_handle(overal_timeout_id);
                }
                if let Some(interval_id) = interval_id.take() {
                    window.clear_interval_with_handle(interval_id);
                }

                // TODO
                // observer.disconnect();

                match result {
                    Ok(result) => {
                        resolve
                            .call1(&JsValue::undefined(), &result.into())
                            .expect("Resolve function should be called.");
                    }
                    Err(error) => {
                        reject
                            .call1(&JsValue::undefined(), &error.into())
                            .expect("Reject function should be called.");
                    }
                }
            }
        };

        let check_callback = Rc::new({
            let callback = callback.clone();
            let last_error = last_error.clone();

            move || match callback() {
                Ok(result) => {
                    on_done(Ok(result));
                }
                Err(error) => {
                    last_error.replace(Some(error));
                }
            }
        });

        let check_real_timers_callback = Closure::<dyn Fn()>::new({
            let check_callback = check_callback.clone();

            move || {
                check_callback();
            }
        });

        overal_timeout_id.replace(Some(
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    handle_timeout.as_ref().unchecked_ref(),
                    timeout,
                )
                .expect("Timeout should be started."),
        ));

        interval_id.replace(Some(
            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    check_real_timers_callback.as_ref().unchecked_ref(),
                    interval,
                )
                .expect("Interval should be started."),
        ));

        // TODO: Initialize observer

        check_callback();
    })
}
