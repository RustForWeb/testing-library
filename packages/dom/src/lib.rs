mod config;
mod dom_element_filter;
mod error;
mod events;
mod get_node_text;
mod get_queries_for_element;
mod helpers;
mod label_helpers;
mod matches;
mod pretty_dom;
pub mod queries;
pub mod query_helpers;
mod role_helpers;
mod suggestions;
mod types;
mod util;
mod wait_for;

pub use config::{configure, get_config};
pub use error::QueryError;
pub use events::*;
pub use get_node_text::*;
pub use get_queries_for_element::get_queries_for_element as within;
pub use get_queries_for_element::*;
pub use matches::get_default_normalizer;
pub use pretty_dom::*;
pub use queries::*;
pub use query_helpers::*;
pub use role_helpers::{
    GetRolesOptions, PrettyRolesOptions, get_implicit_aria_roles, get_roles, is_inaccessible,
    log_roles,
};
pub use suggestions::*;
pub use types::*;
pub use wait_for::*;

// TODO: Export useful types from `aria_query`.
#[doc(no_inline)]
pub use aria_query::{AriaRole, AriaRoleDefinitionKey};
