pub mod error;

mod commands;
mod info;
mod io;
mod methods {}
mod variables;

// TODO: (more) public imports for stuff that might be used in the interface
pub use commands::{main_commands, CmdSet, Command};
pub use info::GameInfo;
pub use io::{AdventureIO, FileType};
pub use variables::{ASType, ASVariable, KeyVar};
