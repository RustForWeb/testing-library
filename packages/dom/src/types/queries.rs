use crate::types::Matcher;

#[derive(Clone, Default)]
pub struct ByRoleOptionsValue {
    pub now: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub text: Option<Matcher>,
}

impl ByRoleOptionsValue {
    pub fn now(mut self, value: f64) -> Self {
        self.now = Some(value);
        self
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn text<M: Into<Matcher>>(mut self, value: M) -> Self {
        self.text = Some(value.into());
        self
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum ByRoleOptionsCurrent {
    Bool(bool),
    String(String),
}

impl From<bool> for ByRoleOptionsCurrent {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&str> for ByRoleOptionsCurrent {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for ByRoleOptionsCurrent {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[derive(Clone, Default)]
pub struct ByRoleOptions {
    pub suggest: Option<bool>,
    pub hidden: Option<bool>,
    pub selected: Option<bool>,
    pub busy: Option<bool>,
    pub checked: Option<bool>,
    pub pressed: Option<bool>,
    pub current: Option<ByRoleOptionsCurrent>,
    pub expanded: Option<bool>,
    pub level: Option<usize>,
    pub value: Option<ByRoleOptionsValue>,
    pub query_fallbacks: Option<bool>,
    pub name: Option<Matcher>,
    pub description: Option<Matcher>,
}

impl ByRoleOptions {
    pub fn suggest(mut self, value: bool) -> Self {
        self.suggest = Some(value);
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        self.hidden = Some(value);
        self
    }

    pub fn selected(mut self, value: bool) -> Self {
        self.selected = Some(value);
        self
    }

    pub fn busy(mut self, value: bool) -> Self {
        self.busy = Some(value);
        self
    }

    pub fn checked(mut self, value: bool) -> Self {
        self.checked = Some(value);
        self
    }

    pub fn pressed(mut self, value: bool) -> Self {
        self.pressed = Some(value);
        self
    }

    pub fn current<C: Into<ByRoleOptionsCurrent>>(mut self, value: C) -> Self {
        self.current = Some(value.into());
        self
    }

    pub fn expanded(mut self, value: bool) -> Self {
        self.expanded = Some(value);
        self
    }

    pub fn level(mut self, value: usize) -> Self {
        self.level = Some(value);
        self
    }

    pub fn value(mut self, value: ByRoleOptionsValue) -> Self {
        self.value = Some(value);
        self
    }

    pub fn query_fallbacks(mut self, value: bool) -> Self {
        self.query_fallbacks = Some(value);
        self
    }

    pub fn name<M: Into<Matcher>>(mut self, value: M) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description<M: Into<Matcher>>(mut self, value: M) -> Self {
        self.description = Some(value.into());
        self
    }
}
