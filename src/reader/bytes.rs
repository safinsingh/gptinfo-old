use crate::Error;
use anyhow::Result;
use colored::ColoredString;

pub(crate) fn format_bytes(end: u64, start: u64) -> String {
	let raw = (end - start + 1) * 512;
	match raw {
		// 1024 bytes in a MB
		1..=1023 => {
			format!("{}B", raw)
		}
		// 1048576 bytes in a KB
		1024..=1048575 => {
			format!("{}K", raw / 1024)
		}
		// 1073741824 bytes in a GB
		1048576..=1073741823 => {
			format!("{}M", raw / 1024 / 1024)
		}
		// 1099511627776 bytes in a TB
		1073741824..=1099511627775 => {
			format!("{}G", raw / 1024 / 1024 / 1024)
		}
		// 1125899906842624 bytes in a PB
		1099511627776..=1125899906842624 => {
			format!("{}T", raw / 1024 / 1024 / 1024 / 1024)
		}
		// if you have more than 1023 terabytes of
		// storage... go do something useful instead
		// of reading this
		x => panic!("a meaningful error {}", x),
	}
}

pub(crate) fn string_from_bytes(bytes: &[u8; 72]) -> Result<ColoredString> {
	let (front, slice, back) = unsafe { bytes.align_to::<u16>() };
	if front.is_empty() && back.is_empty() {
		Ok(ColoredString::from(String::from_utf16(slice)?.as_str()))
	} else {
		Err(Error::UTF16.into())
	}
}

pub(crate) fn guid_from_bytes(
	kind_p1: &[u8; 4],
	kind_p2: &[u8; 2],
	kind_p3: &[u8; 2],
	kind_p4: &[u8; 2],
	kind_p5: &[u8; 6],
) -> String {
	let mut guid = String::new();
	for ch in kind_p1 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p2 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p3 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p4 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p5 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.to_ascii_uppercase()
}
