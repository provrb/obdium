mod cmd;
pub mod dicts;
pub mod mid;
pub mod obd;
mod pid;
mod replay;
mod response;
pub mod scalar;
pub mod vin;

use std::sync::atomic::AtomicUsize;

pub use cmd::*;
pub use obd::*;
pub use pid::*;
pub use response::*;

const CODE_DESC_DB_PATH: &str = "./data/code-descriptions.sqlite";
const MODE22_PIDS_DB_PATH: &str = "./data/model-pids.sqlite";

/// Whether or not to pause OBD threads
/// Keeps track of how many threads want to pause obd
/// If > 0, do not do low priority actions
pub static PAUSE_OBD_COUNT: AtomicUsize = AtomicUsize::new(0);
