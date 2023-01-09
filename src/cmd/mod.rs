//! Subcommand modules for the `mdbook` binary.

pub mod build;
pub mod clean;
pub mod command_prelude;
pub mod init;
#[cfg(feature = "serve")]
pub mod serve;
pub mod test;
#[cfg(feature = "watch")]
pub mod watch;

use clap::ArgMatches;
use log::{error, info};
use std::{env, ffi::OsStr, path::PathBuf};

fn get_book_dir(args: &ArgMatches) -> PathBuf {
    if let Some(p) = args.get_one::<PathBuf>("dir") {
        // Check if path is relative from current dir, or absolute...
        if p.is_relative() {
            env::current_dir().unwrap().join(p)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().expect("Unable to determine the current directory")
    }
}

fn open<P: AsRef<OsStr>>(path: P) {
    info!("Opening web browser");
    if let Err(e) = opener::open(path) {
        error!("Error opening web browser: {}", e);
    }
}
