use crate::errors::Error;

use anyhow::Result;

pub(crate) fn match_partition_guid<'a>(
	kind_p1: &[u8; 4],
	kind_p2: &[u8; 2],
	kind_p3: &[u8; 2],
	kind_p4: &[u8; 8],
) -> Result<&'a str> {
	match (kind_p1, kind_p2, kind_p3, kind_p4) {
		(
			&[0x00, 0x00, 0x00, 0x00],
			&[0x00, 0x00],
			&[0x00, 0x00],
			&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
		) => Ok("Unused entry"),
		(
			&[0x02, 0x4D, 0xEE, 0x41],
			&[0x33, 0xE7],
			&[0x11, 0xD3],
			&[0x9F, 0xF3, 0x81, 0xC7, 0x08, 0x00, 0x69, 0x9D],
		) => Ok("MBR partition scheme"),
		(
			&[0xC1, 0x2A, 0x73, 0x28],
			&[0xF8, 0x1F],
			&[0x11, 0xD2],
			&[0x3B, 0xC9, 0x3E, 0xC9, 0xA0, 0x00, 0x4B, 0xBA],
		) => Ok("EFI System partition"),
		(
			&[0x21, 0x68, 0x61, 0x48],
			&[0x64, 0x49],
			&[0x6E, 0x6F],
			&[0x49, 0x46, 0x45, 0x64, 0x65, 0x65, 0x4E, 0x74],
		) => Ok("BIOS boot partition"),
		(
			&[0xD3, 0xBF, 0xE2, 0xDE],
			&[0x3D, 0xAF],
			&[0x11, 0xDF],
			&[0x93, 0x95, 0xD8, 0x56, 0xA5, 0xE3, 0x40, 0xBA],
		) => Ok("Intel Fast Flash (iFFS) partition (for Intel Rapid Start \
		        technology)"),
		(
			&[0xF4, 0x01, 0x97, 0x32],
			&[0x06, 0x6E],
			&[0x4E, 0x12],
			&[0x4F, 0x49, 0x41, 0x56, 0x6C, 0x34, 0x73, 0x82],
		) => Ok("Sony boot partition"),
		(
			&[0xBF, 0xBF, 0xAF, 0xE7],
			&[0xA3, 0x4F],
			&[0x44, 0x8A],
			&[0x22, 0x6C, 0x73, 0xEB, 0x13, 0x62, 0x5B, 0x9A],
		) => Ok("Lenovo boot partition"),
		(
			&[0xE3, 0xC9, 0xE3, 0x16],
			&[0x0B, 0x5C],
			&[0x4D, 0xB8],
			&[0xAE, 0x15, 0x02, 0xF0, 0x2D, 0xF9, 0x7D, 0x81],
		) => Ok("Windows Microsoft Reserved Partition (MSR)"),
		(
			&[0xEB, 0xD0, 0xA0, 0xA2],
			&[0xB9, 0xE5],
			&[0x44, 0x33],
			&[0xC7, 0x99, 0x26, 0xB7, 0xB6, 0x68, 0xC0, 0x87],
		) => Ok("Windows Basic data partition"),
		(
			&[0x58, 0x08, 0xC8, 0xAA],
			&[0x7E, 0x8F],
			&[0x42, 0xE0],
			&[0xB3, 0xCF, 0x34, 0x04, 0xE9, 0xE1, 0xD2, 0x85],
		) => Ok("Windows Logical Disk Manager (LDM) metadata partition"),
		(
			&[0xAF, 0x9B, 0x60, 0xA0],
			&[0x14, 0x31],
			&[0x4F, 0x62],
			&[0xAD, 0x69, 0x4A, 0x71, 0x11, 0x33, 0x68, 0xBC],
		) => Ok("Windows Logical Disk Manager data partition"),
		(
			&[0xDE, 0x94, 0xBB, 0xA4],
			&[0x06, 0xD1],
			&[0x4D, 0x40],
			&[0xAC, 0xD6, 0x79, 0x01, 0xD5, 0xBF, 0x6A, 0xA1],
		) => Ok("Windows Recovery Environment"),
		(
			&[0x37, 0xAF, 0xFC, 0x90],
			&[0xEF, 0x7D],
			&[0x4E, 0x96],
			&[0x74, 0xB1, 0x55, 0xE0, 0x7A, 0x2D, 0xC3, 0x91],
		) => Ok("Windows IBM General Parallel File System (GPFS) partition"),
		(
			&[0xE7, 0x5C, 0xAF, 0x8F],
			&[0xF6, 0x80],
			&[0x4C, 0xEE],
			&[0x2D, 0xFC, 0x6E, 0xE5, 0x01, 0xB0, 0xA3, 0xAF],
		) => Ok("Windows Storage Spaces partition"),
		(
			&[0x55, 0x8D, 0x43, 0xC5],
			&[0xA1, 0xAC],
			&[0x43, 0xC0],
			&[0xD1, 0x23, 0x29, 0x2B, 0x47, 0xD1, 0xC8, 0xAA],
		) => Ok("Windows Storage Replica partition"),
		(
			&[0x75, 0x89, 0x4C, 0x1E],
			&[0x3A, 0xEB],
			&[0x11, 0xD3],
			&[0x00, 0x00, 0x00, 0xA0, 0x03, 0x7B, 0xC1, 0xB7],
		) => Ok("HP-UX Data partition"),
		(
			&[0xE2, 0xA1, 0xE7, 0x28],
			&[0x32, 0xE3],
			&[0x11, 0xD6],
			&[0x00, 0x00, 0x00, 0xA0, 0x03, 0x7B, 0x82, 0xA6],
		) => Ok("HP-UX Service partition"),
		(
			&[0x0F, 0xC6, 0x3D, 0xAF],
			&[0x84, 0x83],
			&[0x47, 0x72],
			&[0xE4, 0x7D, 0x47, 0xD8, 0x69, 0x3D, 0x79, 0x8E],
		) => Ok("Linux filesystem data"),
		(
			&[0xA1, 0x9D, 0x88, 0x0F],
			&[0x05, 0xFC],
			&[0x4D, 0x3B],
			&[0x1E, 0x91, 0x84, 0x0F, 0x3F, 0x74, 0x06, 0xA0],
		) => Ok("Linux RAID partition"),
		(
			&[0x44, 0x47, 0x95, 0x40],
			&[0xF2, 0x97],
			&[0x41, 0xB2],
			&[0x8A, 0x45, 0xF0, 0xD5, 0x31, 0xD1, 0xF7, 0x9A],
		) => Ok("Linux Root partition (x86)"),
		(
			&[0x4F, 0x68, 0xBC, 0xE3],
			&[0xE8, 0xCD],
			&[0x4D, 0xB1],
			&[0x09, 0xB7, 0x84, 0xF9, 0xCA, 0xFB, 0xE7, 0x96],
		) => Ok("Linux Root partition (x86-64)"),
		(
			&[0x69, 0xDA, 0xD7, 0x10],
			&[0x2C, 0xE4],
			&[0x4E, 0x3C],
			&[0xD3, 0xBE, 0x9A, 0xD4, 0xA1, 0x21, 0x6C, 0xB1],
		) => Ok("Linux Root partition (32-bit ARM)"),
		(
			&[0xB9, 0x21, 0xB0, 0x45],
			&[0x1D, 0xF0],
			&[0x41, 0xC3],
			&[0xAE, 0x3F, 0x0D, 0x28, 0x6F, 0x4C, 0x44, 0xAF],
		) => Ok("Linux Root partition (64-bit ARM/AArch64)"),
		(
			&[0xBC, 0x13, 0xC2, 0xFF],
			&[0x59, 0xE6],
			&[0x42, 0x62],
			&[0x72, 0x71, 0x6F, 0xFD, 0x75, 0xB2, 0x52, 0xA3],
		) => Ok("Linux /boot partition"),
		(
			&[0x06, 0x57, 0xFD, 0x6D],
			&[0xA4, 0xAB],
			&[0x43, 0xC4],
			&[0x4F, 0x4F, 0x4B, 0xC8, 0x33, 0x09, 0xE5, 0x84],
		) => Ok("Linux Swap partition"),
		(
			&[0xE6, 0xD6, 0xD3, 0x79],
			&[0xF5, 0x07],
			&[0x44, 0xC2],
			&[0x28, 0xF9, 0x3D, 0x2A, 0x8F, 0x23, 0x3C, 0xA2],
		) => Ok("Linux Logical Volume Manager (LVM) partition"),
		(
			&[0x93, 0x3A, 0xC7, 0xE1],
			&[0x2E, 0xB4],
			&[0x4F, 0x13],
			&[0x15, 0xF9, 0xAE, 0xE2, 0x14, 0x0E, 0x44, 0xB8],
		) => Ok("Linux /home partition"),
		(
			&[0x3B, 0x8F, 0x84, 0x25],
			&[0x20, 0xE0],
			&[0x4F, 0x3B],
			&[0xE8, 0x98, 0x6F, 0xA7, 0x25, 0x1A, 0x7F, 0x90],
		) => Ok("Linux /srv (server data) partition"),
		(
			&[0x7F, 0xFE, 0xC5, 0xC9],
			&[0x2D, 0x00],
			&[0x49, 0xB7],
			&[0xB7, 0x86, 0x55, 0x0A, 0xA1, 0x3E, 0x41, 0x89],
		) => Ok("Linux Plain dm-crypt partition"),
		(
			&[0xCA, 0x7D, 0x7C, 0xCB],
			&[0x63, 0xED],
			&[0x4C, 0x53],
			&[0xCC, 0x59, 0x60, 0x53, 0x42, 0x17, 0x1C, 0x86],
		) => Ok("Linux LUKS partition"),
		(
			&[0x8D, 0xA6, 0x33, 0x39],
			&[0x00, 0x07],
			&[0x60, 0xC0],
			&[0x08, 0x09, 0x23, 0xC8, 0x3A, 0x08, 0x36, 0xC4],
		) => Ok("Linux Reserved"),
		(
			&[0x83, 0xBD, 0x6B, 0x9D],
			&[0x7F, 0x41],
			&[0x11, 0xDC],
			&[0x0F, 0x4F, 0xB8, 0x60, 0x15, 0x00, 0x0B, 0xBE],
		) => Ok("FreeBSD Boot partition"),
		(
			&[0x51, 0x6E, 0x7C, 0xB4],
			&[0x6E, 0xCF],
			&[0x11, 0xD6],
			&[0x2B, 0x71, 0x09, 0x2D, 0x02, 0x00, 0xF8, 0x8F],
		) => Ok("FreeBSD Data partition"),
		(
			&[0x51, 0x6E, 0x7C, 0xB5],
			&[0x6E, 0xCF],
			&[0x11, 0xD6],
			&[0x2B, 0x71, 0x09, 0x2D, 0x02, 0x00, 0xF8, 0x8F],
		) => Ok("FreeBSD Swap partition"),
		(
			&[0x51, 0x6E, 0x7C, 0xB6],
			&[0x6E, 0xCF],
			&[0x11, 0xD6],
			&[0x2B, 0x71, 0x09, 0x2D, 0x02, 0x00, 0xF8, 0x8F],
		) => Ok("FreeBSD Unix File System (UFS) partition"),
		(
			&[0x51, 0x6E, 0x7C, 0xB8],
			&[0x6E, 0xCF],
			&[0x11, 0xD6],
			&[0x2B, 0x71, 0x09, 0x2D, 0x02, 0x00, 0xF8, 0x8F],
		) => Ok("FreeBSD Vinum volume manager partition"),
		(
			&[0x51, 0x6E, 0x7C, 0xBA],
			&[0x6E, 0xCF],
			&[0x11, 0xD6],
			&[0x2B, 0x71, 0x09, 0x2D, 0x02, 0x00, 0xF8, 0x8F],
		) => Ok("FreeBSD ZFS partition"),
		(
			&[0x48, 0x46, 0x53, 0x00],
			&[0x00, 0x00],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Hierarchical File System Plus (HFS+) partition"),
		(
			&[0x7C, 0x34, 0x57, 0xEF],
			&[0x00, 0x00],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple APFS container"),
		// (
		// 	&[0x7C, 0x34, 0x57, 0xEF],
		// 	&[0x00, 0x00],
		// 	&[0x11, 0xAA],
		// 	&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		// ) => Ok("Darwin APFS FileVault volume container"),
		(
			&[0x55, 0x46, 0x53, 0x00],
			&[0x00, 0x00],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple UFS container"),
		(
			&[0x6A, 0x89, 0x8C, 0xC3],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Darwin ZFS"),
		(
			&[0x52, 0x41, 0x49, 0x44],
			&[0x00, 0x00],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple RAID partition"),
		(
			&[0x52, 0x41, 0x49, 0x44],
			&[0x5F, 0x4F],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple RAID partition, offline"),
		(
			&[0x42, 0x6F, 0x6F, 0x74],
			&[0x00, 0x00],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple Boot partition (Recovery HD)"),
		(
			&[0x4C, 0x61, 0x62, 0x65],
			&[0x6C, 0x00],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple Label"),
		(
			&[0x52, 0x65, 0x63, 0x6F],
			&[0x76, 0x65],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple TV Recovery partition"),
		(
			&[0x53, 0x74, 0x6F, 0x72],
			&[0x61, 0x67],
			&[0x11, 0xAA],
			&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		) => Ok("Darwin Apple Core Storage Container"),
		// (
		// 	&[0x53, 0x74, 0x6F, 0x72],
		// 	&[0x61, 0x67],
		// 	&[0x11, 0xAA],
		// 	&[0xAC, 0xEC, 0x43, 0x65, 0x30, 0x00, 0x11, 0xAA],
		// ) => Ok("Darwin HFS+ FileVault volume container"),
		(
			&[0xB6, 0xFA, 0x30, 0xDA],
			&[0x92, 0xD2],
			&[0x4A, 0x9A],
			&[0x00, 0x62, 0x48, 0xC6, 0x1E, 0x87, 0xF1, 0x96],
		) => Ok("Darwin SoftRAID_Status"),
		(
			&[0x2E, 0x31, 0x34, 0x65],
			&[0x19, 0xB9],
			&[0x46, 0x3F],
			&[0x01, 0x38, 0x77, 0x93, 0x79, 0x8A, 0x26, 0x81],
		) => Ok("Darwin SoftRAID_Scratch"),
		(
			&[0xFA, 0x70, 0x9C, 0x7E],
			&[0x65, 0xB1],
			&[0x45, 0x93],
			&[0x02, 0x9B, 0xDE, 0x61, 0x1D, 0xE7, 0xD5, 0xBF],
		) => Ok("Darwin SoftRAID_Volume"),
		(
			&[0xBB, 0xBA, 0x6D, 0xF5],
			&[0xF4, 0x6F],
			&[0x4A, 0x89],
			&[0x03, 0x75, 0x72, 0xB2, 0x65, 0x87, 0x59, 0x8F],
		) => Ok("Darwin SoftRAID_Cache"),
		(
			&[0x6A, 0x82, 0xCB, 0x45],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris illumos Boot partition"),
		(
			&[0x6A, 0x85, 0xCF, 0x4D],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Root partition"),
		(
			&[0x6A, 0x87, 0xC4, 0x6F],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Swap partition"),
		(
			&[0x6A, 0x8B, 0x64, 0x2B],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Backup partition"),
		// (
		// 	&[0x6A, 0x89, 0x8C, 0xC3],
		// 	&[0x1D, 0xD2],
		// 	&[0x11, 0xB2],
		// 	&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		// ) => Ok("Solaris /usr partition"),
		(
			&[0x6A, 0x8E, 0xF2, 0xE9],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris /var partition"),
		(
			&[0x6A, 0x90, 0xBA, 0x39],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris /home partition"),
		(
			&[0x6A, 0x92, 0x83, 0xA5],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Alternate sector"),
		(
			&[0x6A, 0x94, 0x5A, 0x3B],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Reserved partition"),
		(
			&[0x6A, 0x96, 0x30, 0xD1],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Reserved partition"),
		(
			&[0x6A, 0x98, 0x07, 0x67],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Reserved partition"),
		(
			&[0x6A, 0x96, 0x23, 0x7F],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Reserved partition"),
		(
			&[0x6A, 0x8D, 0x2A, 0xC7],
			&[0x1D, 0xD2],
			&[0x11, 0xB2],
			&[0x31, 0x66, 0x73, 0x20, 0x00, 0x08, 0xA6, 0x99],
		) => Ok("Solaris Reserved partition"),
		(
			&[0x49, 0xF4, 0x8D, 0x32],
			&[0xB1, 0x0E],
			&[0x11, 0xDC],
			&[0x48, 0x96, 0x87, 0xD1, 0x19, 0x00, 0x9B, 0xB9],
		) => Ok("NetBSD Swap partition"),
		(
			&[0x49, 0xF4, 0x8D, 0x5A],
			&[0xB1, 0x0E],
			&[0x11, 0xDC],
			&[0x48, 0x96, 0x87, 0xD1, 0x19, 0x00, 0x9B, 0xB9],
		) => Ok("NetBSD FFS partition"),
		(
			&[0x49, 0xF4, 0x8D, 0x82],
			&[0xB1, 0x0E],
			&[0x11, 0xDC],
			&[0x48, 0x96, 0x87, 0xD1, 0x19, 0x00, 0x9B, 0xB9],
		) => Ok("NetBSD LFS partition"),
		(
			&[0x49, 0xF4, 0x8D, 0xAA],
			&[0xB1, 0x0E],
			&[0x11, 0xDC],
			&[0x48, 0x96, 0x87, 0xD1, 0x19, 0x00, 0x9B, 0xB9],
		) => Ok("NetBSD RAID partition"),
		(
			&[0x2D, 0xB5, 0x19, 0xC4],
			&[0xB1, 0x0F],
			&[0x11, 0xDC],
			&[0x48, 0x96, 0x87, 0xD1, 0x19, 0x00, 0x9B, 0xB9],
		) => Ok("NetBSD Concatenated partition"),
		(
			&[0x2D, 0xB5, 0x19, 0xEC],
			&[0xB1, 0x0F],
			&[0x11, 0xDC],
			&[0x48, 0x96, 0x87, 0xD1, 0x19, 0x00, 0x9B, 0xB9],
		) => Ok("NetBSD Encrypted partition"),
		(
			&[0xFE, 0x3A, 0x2A, 0x5D],
			&[0x4F, 0x32],
			&[0x41, 0xA7],
			&[0x09, 0xA3, 0x85, 0x32, 0xCC, 0xAC, 0x25, 0xB7],
		) => Ok("Chrome OS kernel"),
		(
			&[0x3C, 0xB8, 0xE2, 0x02],
			&[0x3B, 0x7E],
			&[0x47, 0xDD],
			&[0xEC, 0xFC, 0x3C, 0xA1, 0xF2, 0x7F, 0x3C, 0x8A],
		) => Ok("Chrome OS rootfs"),
		(
			&[0x2E, 0x0A, 0x75, 0x3D],
			&[0x9E, 0x48],
			&[0x43, 0xB0],
			&[0x5E, 0x1B, 0xCB, 0x92, 0x51, 0xB1, 0x37, 0x83],
		) => Ok("Chrome OS future use"),
		(
			&[0x5D, 0xFB, 0xF5, 0xF4],
			&[0x28, 0x48],
			&[0x4B, 0xAC],
			&[0xA6, 0x45, 0xB7, 0x20, 0x9A, 0x0D, 0x5E, 0xAA],
		) => Ok("Container Linux by CoreOS /usr partition (coreos-usr)"),
		(
			&[0x38, 0x84, 0xDD, 0x41],
			&[0x85, 0x82],
			&[0x44, 0x04],
			&[0x0E, 0xF5, 0x2D, 0x4F, 0xB8, 0xE9, 0xA8, 0xB9],
		) => Ok("Container Linux by CoreOS Resizable rootfs (coreos-resize)"),
		(
			&[0xC9, 0x5D, 0xC2, 0x1A],
			&[0xDF, 0x0E],
			&[0x43, 0x40],
			&[0xE0, 0x03, 0x9A, 0xFA, 0xCB, 0x26, 0x7B, 0x8D],
		) => Ok("Container Linux by CoreOS OEM customizations (coreos-reserved)"),
		(
			&[0xBE, 0x90, 0x67, 0xB9],
			&[0xEA, 0x49],
			&[0x4F, 0x15],
			&[0x18, 0x18, 0x9E, 0x8C, 0x6F, 0xF3, 0xF6, 0xB4],
		) => Ok("Container Linux by CoreOS Root filesystem on RAID \
		        (coreos-root-raid)"),
		(
			&[0x42, 0x46, 0x53, 0x31],
			&[0x3B, 0xA3],
			&[0x10, 0xF1],
			&[0x21, 0x75, 0x6B, 0x69, 0x61, 0x48, 0x2A, 0x80],
		) => Ok("Haiku BFS"),
		(
			&[0x85, 0xD5, 0xE4, 0x5E],
			&[0x23, 0x7C],
			&[0x11, 0xE1],
			&[0xA7, 0xC3, 0x7F, 0x8F, 0x9A, 0xE8, 0xB3, 0xB4],
		) => Ok("MidnightBSD Boot partition"),
		(
			&[0x85, 0xD5, 0xE4, 0x5A],
			&[0x23, 0x7C],
			&[0x11, 0xE1],
			&[0xA7, 0xC3, 0x7F, 0x8F, 0x9A, 0xE8, 0xB3, 0xB4],
		) => Ok("MidnightBSD Data partition"),
		(
			&[0x85, 0xD5, 0xE4, 0x5B],
			&[0x23, 0x7C],
			&[0x11, 0xE1],
			&[0xA7, 0xC3, 0x7F, 0x8F, 0x9A, 0xE8, 0xB3, 0xB4],
		) => Ok("MidnightBSD Swap partition"),
		(
			&[0x03, 0x94, 0xEF, 0x8B],
			&[0x23, 0x7E],
			&[0x11, 0xE1],
			&[0xA7, 0xC3, 0x7F, 0x8F, 0x9A, 0xE8, 0xB3, 0xB4],
		) => Ok("MidnightBSD Unix File System (UFS) partition"),
		(
			&[0x85, 0xD5, 0xE4, 0x5C],
			&[0x23, 0x7C],
			&[0x11, 0xE1],
			&[0xA7, 0xC3, 0x7F, 0x8F, 0x9A, 0xE8, 0xB3, 0xB4],
		) => Ok("MidnightBSD Vinum volume manager partition"),
		(
			&[0x85, 0xD5, 0xE4, 0x5D],
			&[0x23, 0x7C],
			&[0x11, 0xE1],
			&[0xA7, 0xC3, 0x7F, 0x8F, 0x9A, 0xE8, 0xB3, 0xB4],
		) => Ok("MidnightBSD ZFS partition"),
		(
			&[0x45, 0xB0, 0x96, 0x9E],
			&[0x9B, 0x03],
			&[0x4F, 0x30],
			&[0x06, 0xF1, 0xEF, 0x0C, 0xB8, 0xB4, 0xC6, 0xB4],
		) => Ok("Ceph Journal"),
		(
			&[0x45, 0xB0, 0x96, 0x9E],
			&[0x9B, 0x03],
			&[0x4F, 0x30],
			&[0x06, 0xF1, 0xEF, 0x0C, 0xC0, 0x5E, 0xC6, 0xB4],
		) => Ok("Ceph dm-crypt journal"),
		(
			&[0x4F, 0xBD, 0x7E, 0x29],
			&[0x9D, 0x25],
			&[0x41, 0xB8],
			&[0x5D, 0xF0, 0xEF, 0x0C, 0x2C, 0x06, 0xD0, 0xAF],
		) => Ok("Ceph OSD"),
		(
			&[0x4F, 0xBD, 0x7E, 0x29],
			&[0x9D, 0x25],
			&[0x41, 0xB8],
			&[0x5D, 0xF0, 0xEF, 0x0C, 0xC0, 0x5E, 0xD0, 0xAF],
		) => Ok("Ceph dm-crypt OSD"),
		(
			&[0x89, 0xC5, 0x7F, 0x98],
			&[0x2F, 0xE5],
			&[0x4D, 0xC0],
			&[0xBE, 0xF2, 0xEF, 0x0C, 0xAD, 0xF3, 0xC1, 0x89],
		) => Ok("Ceph Disk in creation"),
		(
			&[0x89, 0xC5, 0x7F, 0x98],
			&[0x2F, 0xE5],
			&[0x4D, 0xC0],
			&[0xBE, 0xF2, 0xEF, 0x0C, 0xC0, 0x5E, 0xC1, 0x89],
		) => Ok("Ceph dm-crypt disk in creation"),
		(
			&[0xCA, 0xFE, 0xCA, 0xFE],
			&[0x9B, 0x03],
			&[0x4F, 0x30],
			&[0x06, 0xF1, 0xEF, 0x0C, 0xB8, 0xB4, 0xC6, 0xB4],
		) => Ok("Ceph Block"),
		(
			&[0x30, 0xCD, 0x08, 0x09],
			&[0xC2, 0xB2],
			&[0x49, 0x9C],
			&[0x76, 0x98, 0x52, 0x78, 0x6B, 0x2D, 0x79, 0x88],
		) => Ok("Ceph Block DB"),
		(
			&[0x5C, 0xE1, 0x7F, 0xCE],
			&[0x40, 0x87],
			&[0x41, 0x69],
			&[0xF9, 0x73, 0x84, 0xC5, 0x6C, 0x05, 0xFF, 0xB7],
		) => Ok("Ceph Block write-ahead log"),
		(
			&[0xFB, 0x3A, 0xAB, 0xF9],
			&[0xD2, 0x5F],
			&[0x47, 0xCC],
			&[0x6B, 0x49, 0x16, 0x18, 0x1D, 0x72, 0x5E, 0xBF],
		) => Ok("Ceph Lockbox for dm-crypt keys"),
		(
			&[0x4F, 0xBD, 0x7E, 0x29],
			&[0x8A, 0xE0],
			&[0x49, 0x82],
			&[0x60, 0xF5, 0x7A, 0x86, 0x8D, 0x5A, 0x9D, 0xBF],
		) => Ok("Ceph Multipath OSD"),
		(
			&[0x45, 0xB0, 0x96, 0x9E],
			&[0x8A, 0xE0],
			&[0x49, 0x82],
			&[0x60, 0xF5, 0x7A, 0x86, 0x8D, 0x5A, 0x9D, 0xBF],
		) => Ok("Ceph Multipath journal"),
		(
			&[0xCA, 0xFE, 0xCA, 0xFE],
			&[0x8A, 0xE0],
			&[0x49, 0x82],
			&[0x60, 0xF5, 0x7A, 0x86, 0x8D, 0x5A, 0x9D, 0xBF],
		) => Ok("Ceph Multipath block"),
		(
			&[0x7F, 0x4A, 0x66, 0x6A],
			&[0x16, 0xF3],
			&[0x47, 0xA2],
			&[0x6C, 0x3F, 0xD0, 0xF4, 0x2E, 0x15, 0x45, 0x84],
		) => Ok("Ceph Multipath block"),
		(
			&[0xEC, 0x6D, 0x63, 0x85],
			&[0xE3, 0x46],
			&[0x45, 0xDC],
			&[0x61, 0x32, 0x8B, 0x7C, 0x2A, 0xDA, 0x91, 0xBE],
		) => Ok("Ceph Multipath block DB"),
		(
			&[0x01, 0xB4, 0x1E, 0x1B],
			&[0x00, 0x2A],
			&[0x45, 0x3C],
			&[0x8F, 0xFF, 0x89, 0x39, 0x79, 0x88, 0x17, 0x9F],
		) => Ok("Ceph Multipath block write-ahead log"),
		(
			&[0xCA, 0xFE, 0xCA, 0xFE],
			&[0x9B, 0x03],
			&[0x4F, 0x30],
			&[0x06, 0xF1, 0xEF, 0x0C, 0xC0, 0x5E, 0xC6, 0xB4],
		) => Ok("Ceph dm-crypt block"),
		(
			&[0x93, 0xB0, 0x05, 0x2D],
			&[0x02, 0xD9],
			&[0x4D, 0x8A],
			&[0xC3, 0xFB, 0x4D, 0xEE, 0xA3, 0x33, 0x3B, 0xA4],
		) => Ok("Ceph dm-crypt block DB"),
		(
			&[0x30, 0x6E, 0x86, 0x83],
			&[0x4F, 0xE2],
			&[0x43, 0x30],
			&[0x66, 0x69, 0xC1, 0x17, 0xA9, 0x00, 0xC0, 0xB7],
		) => Ok("Ceph dm-crypt block write-ahead log"),
		(
			&[0x45, 0xB0, 0x96, 0x9E],
			&[0x9B, 0x03],
			&[0x4F, 0x30],
			&[0x06, 0xF1, 0xEF, 0x5C, 0x86, 0x35, 0xC6, 0xB4],
		) => Ok("Ceph dm-crypt LUKS journal"),
		(
			&[0xCA, 0xFE, 0xCA, 0xFE],
			&[0x9B, 0x03],
			&[0x4F, 0x30],
			&[0x06, 0xF1, 0xEF, 0x5C, 0x86, 0x35, 0xC6, 0xB4],
		) => Ok("Ceph dm-crypt LUKS block"),
		(
			&[0x16, 0x64, 0x18, 0xDA],
			&[0xC4, 0x69],
			&[0x40, 0x22],
			&[0x76, 0xF1, 0x37, 0xFD, 0x0A, 0xB3, 0xF4, 0xAD],
		) => Ok("Ceph dm-crypt LUKS block DB"),
		(
			&[0x86, 0xA3, 0x20, 0x90],
			&[0x36, 0x47],
			&[0x40, 0xB9],
			&[0x86, 0xAA, 0x73, 0xC5, 0xD8, 0x38, 0xBD, 0xBB],
		) => Ok("Ceph dm-crypt LUKS block write-ahead log"),
		(
			&[0x4F, 0xBD, 0x7E, 0x29],
			&[0x9D, 0x25],
			&[0x41, 0xB8],
			&[0x5D, 0xF0, 0xEF, 0x5C, 0x86, 0x35, 0xD0, 0xAF],
		) => Ok("Ceph dm-crypt LUKS OSD"),
		(
			&[0x82, 0x4C, 0xC7, 0xA0],
			&[0x36, 0xA8],
			&[0x11, 0xE3],
			&[0x61, 0x3F, 0xAD, 0x19, 0x25, 0x95, 0x0A, 0x89],
		) => Ok("OpenBSD Data partition"),
		(
			&[0xCE, 0xF5, 0xA9, 0xAD],
			&[0x73, 0xBC],
			&[0x46, 0x01],
			&[0xA1, 0x21, 0xE3, 0xEE, 0xEE, 0xCD, 0xF3, 0x89],
		) => Ok("QNX Power-safe (QNX6) file system"),
		(
			&[0xC9, 0x18, 0x18, 0xF9],
			&[0x80, 0x25],
			&[0x47, 0xAF],
			&[0x2C, 0x0C, 0x00, 0xD7, 0x30, 0xF0, 0xD2, 0x89],
		) => Ok("Plan 9 Plan 9 partition"),
		(
			&[0x9D, 0x27, 0x53, 0x80],
			&[0x40, 0xAD],
			&[0x11, 0xDB],
			&[0xB8, 0xD1, 0x11, 0x29, 0x0C, 0x00, 0x97, 0xBF],
		) => Ok("VMware ESX vmkcore (coredump partition)"),
		(
			&[0xAA, 0x31, 0xE0, 0x2A],
			&[0x40, 0x0F],
			&[0x11, 0xDB],
			&[0xB8, 0xD1, 0x11, 0x29, 0x0C, 0x00, 0x90, 0x95],
		) => Ok("VMware VMFS filesystem partition"),
		(
			&[0x91, 0x98, 0xEF, 0xFC],
			&[0x31, 0xC0],
			&[0x11, 0xDB],
			&[0xB8, 0xD1, 0x11, 0x29, 0x0C, 0x00, 0x78, 0x8F],
		) => Ok("VMware Reserved"),
		(
			&[0x25, 0x68, 0x84, 0x5D],
			&[0x23, 0x32],
			&[0x46, 0x75],
			&[0x15, 0x8D, 0x74, 0xA4, 0xA5, 0x8F, 0x39, 0xBC],
		) => Ok("Android-IA Bootloader"),
		(
			&[0x11, 0x4E, 0xAF, 0xFE],
			&[0x15, 0x52],
			&[0x40, 0x22],
			&[0x84, 0xCF, 0x04, 0x36, 0x05, 0x9B, 0x6E, 0xB2],
		) => Ok("Android-IA Bootloader2"),
		(
			&[0x49, 0xA4, 0xD1, 0x7F],
			&[0x93, 0xA3],
			&[0x45, 0xC1],
			&[0x99, 0x25, 0xBE, 0x2E, 0x0B, 0xF5, 0xDE, 0xA0],
		) => Ok("Android-IA Boot"),
		(
			&[0x41, 0x77, 0xC7, 0x22],
			&[0x9E, 0x92],
			&[0x4A, 0xAB],
			&[0x06, 0x55, 0xFD, 0x2B, 0x50, 0x43, 0x44, 0x86],
		) => Ok("Android-IA Recovery"),
		(
			&[0xEF, 0x32, 0xA3, 0x3B],
			&[0xA4, 0x09],
			&[0x48, 0x6C],
			&[0x66, 0x62, 0x1F, 0x71, 0xFB, 0x9F, 0x41, 0x91],
		) => Ok("Android-IA Misc"),
		(
			&[0x20, 0xAC, 0x26, 0xBE],
			&[0x20, 0xB7],
			&[0x11, 0xE3],
			&[0xE9, 0x11, 0x47, 0xB9, 0xFD, 0x6C, 0xC5, 0x84],
		) => Ok("Android-IA Metadata"),
		(
			&[0x38, 0xF4, 0x28, 0xE6],
			&[0xD3, 0x26],
			&[0x42, 0x5D],
			&[0x7C, 0x64, 0x33, 0xA1, 0x0E, 0x6E, 0x40, 0x91],
		) => Ok("Android-IA System"),
		(
			&[0xA8, 0x93, 0xEF, 0x21],
			&[0xE4, 0x28],
			&[0x47, 0x0A],
			&[0xD9, 0xA2, 0x91, 0xFD, 0x68, 0x06, 0x55, 0x9E],
		) => Ok("Android-IA Cache"),
		(
			&[0xDC, 0x76, 0xDD, 0xA9],
			&[0x5A, 0xC1],
			&[0x49, 0x1C],
			&[0x0D, 0x0C, 0x58, 0x91, 0x25, 0xA8, 0x42, 0xAF],
		) => Ok("Android-IA Data"),
		(
			&[0xEB, 0xC5, 0x97, 0xD0],
			&[0x20, 0x53],
			&[0x4B, 0x15],
			&[0xB1, 0x4D, 0x5F, 0xC7, 0xAA, 0xE0, 0x64, 0x8B],
		) => Ok("Android-IA Persistent"),
		(
			&[0xC5, 0xA0, 0xAE, 0xEC],
			&[0x13, 0xEA],
			&[0x11, 0xE5],
			&[0x3C, 0x0C, 0xCA, 0x67, 0x1E, 0x00, 0xB1, 0xA1],
		) => Ok("Vendor"),
		(
			&[0xBD, 0x59, 0x40, 0x8B],
			&[0x45, 0x14],
			&[0x49, 0x0D],
			&[0x78, 0xF3, 0x63, 0xD9, 0x78, 0x98, 0x12, 0xBF],
		) => Ok("Android-IA Config"),
		(
			&[0x8F, 0x68, 0xCC, 0x74],
			&[0xC5, 0xE5],
			&[0x48, 0xDA],
			&[0x80, 0x9C, 0x5E, 0xC1, 0xC8, 0xA0, 0x91, 0xBE],
		) => Ok("Android-IA Factory"),
		(
			&[0x9F, 0xDA, 0xA6, 0xEF],
			&[0x4B, 0x3F],
			&[0x40, 0xD2],
			&[0x7B, 0x88, 0xFB, 0x6B, 0xF1, 0xBF, 0x8D, 0xBA],
		) => Ok("Android-IA Factory (alt)"),
		(
			&[0x76, 0x79, 0x41, 0xD0],
			&[0x20, 0x85],
			&[0x11, 0xE3],
			&[0xE9, 0x11, 0x47, 0xB9, 0xFD, 0x6C, 0x3B, 0xAD],
		) => Ok("Android-IA Fastboot / Tertiary"),
		(
			&[0xAC, 0x6D, 0x79, 0x24],
			&[0xEB, 0x71],
			&[0x4D, 0xF8],
			&[0xFF, 0x48, 0x71, 0xB2, 0x67, 0xE2, 0x8D, 0xB4],
		) => Ok("Android-IA OEM"),
		(
			&[0x19, 0xA7, 0x10, 0xA2],
			&[0xB3, 0xCA],
			&[0x11, 0xE4],
			&[0xCF, 0x9D, 0x88, 0x4B, 0x60, 0x10, 0x26, 0xB0],
		) => Ok("Android 6.0+ ARM Android Meta"),
		(
			&[0x19, 0x3D, 0x1E, 0xA4],
			&[0xB3, 0xCA],
			&[0x11, 0xE4],
			&[0xCF, 0x9D, 0x88, 0x4B, 0x60, 0x10, 0x75, 0xB0],
		) => Ok("Android EXT"),
		(
			&[0x74, 0x12, 0xF7, 0xD5],
			&[0xA1, 0x56],
			&[0x4B, 0x13],
			&[0x25, 0x93, 0x92, 0x74, 0x71, 0x86, 0xDC, 0x81],
		) => Ok("Open Network Install Environment (ONIE) Boot"),
		(
			&[0xD4, 0xE6, 0xE2, 0xCD],
			&[0x44, 0x69],
			&[0x46, 0xF3],
			&[0x49, 0xC1, 0xAF, 0x57, 0xFF, 0x1B, 0xCB, 0xB5],
		) => Ok("Open Network Install Environment (ONIE) Config"),
		(
			&[0x9E, 0x1A, 0x2D, 0x38],
			&[0xC6, 0x12],
			&[0x43, 0x16],
			&[0x8B, 0x5A, 0x1E, 0x52, 0x49, 0x8B, 0x26, 0xAA],
		) => Ok("PowerPC PReP boot"),
		// (
		// 	&[0xBC, 0x13, 0xC2, 0xFF],
		// 	&[0x59, 0xE6],
		// 	&[0x42, 0x62],
		// 	&[0x72, 0x71, 0x6F, 0xFD, 0x75, 0xB2, 0x52, 0xA3],
		// ) => Ok("freedesktop.org OSes (Linux, etc.) Shared boot loader \
		//         configuration"),
		(
			&[0x73, 0x4E, 0x5A, 0xFE],
			&[0xF6, 0x1A],
			&[0x11, 0xE6],
			&[0x71, 0x26, 0x00, 0x1F, 0x36, 0x92, 0x64, 0xBC],
		) => Ok("Atari TOS Basic data partition (GEM, BGM, F32)"),
		(
			&[0x8C, 0x8F, 0x8E, 0xFF],
			&[0xAC, 0x95],
			&[0x47, 0x70],
			&[0x8F, 0xBC, 0x2D, 0x4F, 0x99, 0x21, 0x4A, 0x81],
		) => Ok("VeraCrypt Encrypted data partition"),
		(
			&[0x90, 0xB6, 0xFF, 0x38],
			&[0xB9, 0x8F],
			&[0x43, 0x58],
			&[0xD3, 0x8A, 0x4A, 0x5B, 0xF3, 0x48, 0x1F, 0xA2],
		) => Ok("OS/2 ArcaOS Type 1"),
		_ => Err(Error::UnrecognizedGUID.into()),
	}
}
