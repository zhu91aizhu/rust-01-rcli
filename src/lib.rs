mod opts;
mod process;

pub use process::{process_csv, process_genpass};
pub use opts::{SubCommand, Opts, GenPassOpts};
