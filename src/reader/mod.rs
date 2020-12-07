mod bytes;
mod header;
mod partition;

use crate::{cli::Table, errors::Error, guid};
use anyhow::{Context as _, Result};
use colored::Colorize;
use nix::unistd::Uid;
use std::{fs::File, os::unix::fs::FileExt};

const MBR_OFFSET: u16 = 512;
const GPT_HEADER_SIG: u64 = 0x5452415020494645;
const UNUSED_PARTITION: &str = "Unused entry";

pub(crate) enum ReaderKind {
	Header,
	Entry,
}

impl ReaderKind {
	fn analyze(
		&mut self,
		bytes: &mut [u8; 512],
		loc: &str,
		writer: &mut Table,
		guid: bool,
	) -> Result<()> {
		match self {
			Self::Header => {
				// SAFETY: guaranteed to be safe because we guarantee the layout
				// of the struct
				let entry = unsafe {
					&mut *(bytes.as_ptr() as *mut header::HeaderEntry)
				};

				// Magic signature in the first 8 bytes of LBA1 to verify EFI
				// system integrity
				let signature = u64::from_le_bytes(entry.signature);

				if signature == GPT_HEADER_SIG {
					// Begin disk section
					writer.push_cell("Name".bold());
					if guid {
						writer.push_cell("Unique GUID".bold());
					}

					writer.push_cells(vec![
						"Type".bold(),
						"Start".bold(),
						"End".bold(),
						"Sectors".bold(),
						"Size".bold(),
					]);

					let start = u64::from_le_bytes(entry.first_lba);
					let end = u64::from_le_bytes(entry.last_lba);

					// reverse guid parts for endianness
					entry.kind_p1.reverse();
					entry.kind_p2.reverse();
					entry.kind_p3.reverse();

					writer.push_cell(format!("Disk ({})", loc).as_str().into());
					if guid {
						writer.push_cell(
							bytes::guid_from_bytes(
								&entry.kind_p1,
								&entry.kind_p2,
								&entry.kind_p3,
								&entry.kind_p4,
								&entry.kind_p5,
							)
							.as_str()
							.into(),
						);
					}

					writer.push_cells(vec![
						"Block Device".into(),
						start.to_string().as_str().into(),
						end.to_string().as_str().into(),
						(end + start).to_string().as_str().into(),
						bytes::format_bytes(end, start).as_str().into(),
					]);
					Ok(())
				} else {
					Err(Error::InvalidSignature.into())
				}
			}
			Self::Entry => {
				// SAFETY: guaranteed to be safe because we guarantee the layout
				// of the struct
				let entry = unsafe {
					&mut *(bytes.as_ptr() as *mut partition::PartitionEntry)
				};

				// account for endianness
				entry.kind_p1.reverse();
				entry.kind_p2.reverse();
				entry.kind_p3.reverse();
				entry.kind_p4.reverse();

				// Get partition type from GUID
				let kind = guid::match_partition_guid(
					&entry.kind_p1,
					&entry.kind_p2,
					&entry.kind_p3,
					&entry.kind_p4,
				)?;

				if kind != UNUSED_PARTITION {
					let start = u64::from_le_bytes(entry.first_lba);
					let end = u64::from_le_bytes(entry.last_lba);

					writer.push_cell(bytes::string_from_bytes(&entry.name)?);
					if guid {
						entry.ukind_p1.reverse();
						entry.ukind_p2.reverse();
						entry.ukind_p3.reverse();

						writer.push_cell(
							bytes::guid_from_bytes(
								&entry.ukind_p1,
								&entry.ukind_p2,
								&entry.ukind_p3,
								&entry.ukind_p4,
								&entry.ukind_p5,
							)
							.as_str()
							.into(),
						);
					}

					writer.push_cells(vec![
						kind.into(),
						start.to_string().as_str().into(),
						end.to_string().as_str().into(),
						(end - start + 1).to_string().as_str().into(),
						bytes::format_bytes(end, start).as_str().into(),
					]);
				}
				Ok(())
			}
		}
	}
}

pub(crate) struct Reader<'a> {
	loc: &'a str,
	lba: u16,
	bytes: [u8; 512],
	writer: Table,
	guid: bool,
}

impl<'a> Reader<'a> {
	pub(crate) fn new(loc: &'a str, guid: bool) -> Result<Reader<'a>> {
		if !Uid::effective().is_root() {
			return Err(Error::Root.into());
		}

		Ok(Self {
			loc,
			lba: 1,
			bytes: [0u8; 512],
			writer: Table::new(if guid { 7 } else { 6 }),
			guid,
		})
	}

	pub(crate) fn read(&mut self) -> Result<ReaderKind> {
		let entry_size = match self.lba {
			1 => 512,
			2..=33 => 128,
			pos => return Err(Error::UndefLbaPtr(pos).into()),
		};
		let offset = MBR_OFFSET + entry_size * (self.lba - 1);

		File::open(self.loc)
			.context("Failed to open GPT entry for reading")?
			.read_at(&mut self.bytes, offset.into())
			.with_context(|| {
				format!("Failed read GPT entry at offset: {}b", offset)
			})?;

		match self.lba {
			1 => Ok(ReaderKind::Header),
			2..=33 => Ok(ReaderKind::Entry),
			pos => Err(Error::UndefLbaPtr(pos).into()),
		}
	}

	pub(crate) fn run(mut self) -> Result<()> {
		while let Ok(mut reader) = self.read() {
			reader.analyze(
				&mut self.bytes,
				self.loc,
				&mut self.writer,
				self.guid,
			)?;
			self.lba += 1;
		}

		self.writer.draw();
		Ok(())
	}
}
