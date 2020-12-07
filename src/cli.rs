use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Safin S. <safinsingh.dev@gmail.com>")]
pub(crate) struct Opts {
	#[clap(short, long, default_value = "/dev/sda")]
	/// EFI System to read from
	pub(crate) system: String,

	#[clap(short, long)]
	/// Run `gptinfo` in quiet mode
	pub(crate) quiet: bool,
}
