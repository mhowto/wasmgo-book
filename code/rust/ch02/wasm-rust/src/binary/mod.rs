mod errors;
mod instructions;
mod module;
mod reader;
mod types;

pub use self::reader::decode_file;
pub use errors::{Error, Result};
pub use module::Module;
