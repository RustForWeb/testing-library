use std::ops::Deref;

use web_sys::window;

use crate::{BoundQueries, DocumentOrElement, get_queries_for_element, log_dom};

pub struct Screen(BoundQueries);

impl Screen {
    pub fn debug(&self, elements: Option<Vec<DocumentOrElement>>, max_length: Option<usize>) {
        if let Some(elements) = elements {
            for element in elements {
                log_dom(Some(element), max_length);
            }
        } else {
            log_dom(None, max_length);
        }
    }
}

impl Deref for Screen {
    type Target = BoundQueries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn screen() -> Screen {
    let body = window()
        .and_then(|window| window.document())
        .and_then(|document| document.body())
        .expect("For queries bound to document.body a global document has to be available.");
    Screen(get_queries_for_element(body))
}
