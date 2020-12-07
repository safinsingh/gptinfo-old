use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub(crate) enum Error {
	#[error("Failed to validate signature of EFI partition.")]
	InvalidSignature,

	#[error("Internal: undefined LBA pointer at index `{0}`.")]
	UndefLbaPtr(u16),

	#[error("Unrecognized partition type GUID.")]
	UnrecognizedGUID,

	#[error("Cannot display non-UTF8 UTF16 characters.")]
	UTF16,

	#[error(
		"This binary must be run as root. See `gptinfo --help` for more \
		 options."
	)]
	Root,
}
