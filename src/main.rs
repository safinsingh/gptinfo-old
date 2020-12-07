#[macro_use]
mod macros;
mod cli;
mod errors;
mod guid;
mod reader;

use anyhow::{Context as _, Result};
use clap::Clap;
use errors::Error;
use lazy_static::lazy_static;
use nix::unistd::Uid;

lazy_static! {
	static ref OPTS: cli::Opts = cli::Opts::parse();
}

fn main() -> Result<()> {
	if !Uid::effective().is_root() {
		return Err(Error::Root.into());
	}

	reader::Reader::new(&*OPTS.system)
		.context("Failed to create Reader")?
		.run()?;

	Ok(())
}
