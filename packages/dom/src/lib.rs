mod config;
mod error;
mod get_node_text;
mod get_queries_for_element;
mod matches;
mod pretty_dom;
pub mod queries;
pub mod query_helpers;
mod types;
mod util;

pub use config::{configure, get_config};
pub use get_node_text::*;
pub use get_queries_for_element::*;
pub use matches::get_default_normalizer;
pub use pretty_dom::*;
pub use queries::*;
pub use query_helpers::*;
pub use types::*;
