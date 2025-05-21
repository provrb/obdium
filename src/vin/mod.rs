mod attributes;
mod element_ids;
mod parser;
mod pattern;
mod schema;
mod wmi;

const VPIC_DB_PATH: &str = "./data/vpic.sqlite";

pub use element_ids::*;
pub use parser::*;
