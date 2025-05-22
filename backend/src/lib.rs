pub mod obd;
pub mod vin;
mod cmd;
mod pid;
mod replay;
mod response;
mod scalar;

pub use pid::*;
pub use response::*;
pub use cmd::*;
pub use obd::*;

const CODE_DESC_DB_PATH: &str = "./data/code-descriptions.sqlite";
const MODE22_PIDS_DB_PATH: &str = "./data/model-pids.sqlite";
const RECORDED_REQEUSTS_DIR: &str = "./data/requests.json";