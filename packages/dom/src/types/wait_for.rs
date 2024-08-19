use web_sys::HtmlElement;

#[derive(Clone, Default)]
pub struct WaitForOptions {
    container: Option<HtmlElement>,
    timeout: Option<i32>,
    interval: Option<i32>,
    // on_timeout
    // mutation_observer_init
}

impl WaitForOptions {
    pub fn container(mut self, value: HtmlElement) -> Self {
        self.container = Some(value);
        self
    }

    pub fn timeout(mut self, value: i32) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn interval(mut self, value: i32) -> Self {
        self.interval = Some(value);
        self
    }
}
