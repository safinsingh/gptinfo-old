mod cli;
mod errors;
mod guid;
mod reader;

use anyhow::{Context as _, Result};
use clap::Clap;
use errors::Error;

fn main() -> Result<()> {
	let cli::Opts { device, guid } = cli::Opts::parse();

	reader::Reader::new(&device, guid)
		.context("Failed to create Reader")?
		.run()?;

	Ok(())
}
