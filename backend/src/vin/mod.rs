mod attributes;
mod element_ids;
mod parser;
mod pattern;
mod schema;
mod wmi;

#[deprecated]
pub const VPIC_DB_PATH: &str = "./data/vpic.sqlite";

pub static APP_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn vpic_db_path() -> Option<PathBuf> {
    if let Some(app_data_dir) = APP_DATA_DIR.get() {
        return Some(app_data_dir.join("vpic.sqlite"));
    }
    None
}

use std::{path::PathBuf, sync::OnceLock};

pub use element_ids::*;
pub use parser::*;
