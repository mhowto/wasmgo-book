mod errors;
mod instructions;
mod module;
mod reader;
mod types;
mod dumper;
mod opcodes;

pub use self::reader::decode_file;
pub use errors::{Error, Result};
pub use module::Module;
pub use dumper::Dumper;
pub use opcodes::*;
