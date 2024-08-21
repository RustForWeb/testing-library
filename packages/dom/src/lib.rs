mod config;
mod error;
mod get_node_text;
mod get_queries_for_element;
mod helpers;
mod matches;
mod pretty_dom;
pub mod queries;
pub mod query_helpers;
mod suggestions;
mod types;
mod util;
mod wait_for;

pub use config::{configure, get_config};
pub use error::QueryError;
pub use get_node_text::*;
pub use get_queries_for_element::*;
pub use matches::get_default_normalizer;
pub use pretty_dom::*;
pub use queries::*;
pub use query_helpers::*;
pub use suggestions::*;
pub use types::*;
pub use wait_for::*;
