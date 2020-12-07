pub mod table;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Safin S. <safinsingh.dev@gmail.com>")]
pub(crate) struct Opts {
	#[clap(short, long, default_value = "/dev/sda")]
	/// EFI System to read from
	pub(crate) system: String,

	#[clap(short, long)]
	/// Run `gptinfo` in verbose mode
	pub(crate) verbose: bool,

	#[clap(short, long)]
	/// Show GUID of each partition
	pub(crate) guid: bool,
}
