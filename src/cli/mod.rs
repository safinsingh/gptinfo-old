mod table;

use clap::Clap;
pub(crate) use table::Table;

#[derive(Clap)]
#[clap(version = "1.0", author = "Safin S. <safinsingh.dev@gmail.com>")]
pub(crate) struct Opts {
	#[clap(short, long, default_value = "/dev/sda")]
	/// Block device to read GPT from
	pub(crate) device: String,

	#[clap(short, long)]
	/// Show GUID of each partition
	pub(crate) guid: bool,
}
