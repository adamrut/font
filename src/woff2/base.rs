use crate::woff2::data_converters::convert_uint_base128_to_u32;
use crate::woff2::types::{Woff2Header, Woff2TableRecord, KNOWN_TABLE_TAGS};
use std::mem;

pub fn woff2_read_header(data: &[u8]) -> Woff2Header {
	Woff2Header {
		signature: u32::from_be_bytes(data[0..4].try_into().expect("Failed to read signature")),
		flavor: u32::from_be_bytes(data[4..8].try_into().expect("Failed to read flavor")),
		length: u32::from_be_bytes(data[8..12].try_into().expect("Failed to read length")),
		num_tables: u16::from_be_bytes(data[12..14].try_into().expect("Failed to read num_tables")),
		reserved: u16::from_be_bytes(data[14..16].try_into().expect("Failed to read reserved")),
		total_sfnt_size: u32::from_be_bytes(data[16..20].try_into().expect("Failed to read total_sfnt_size")),
		total_compressed_size: u32::from_be_bytes(data[20..24].try_into().expect("Failed to read total_compressed_size")),
		major_version: u16::from_be_bytes(data[24..26].try_into().expect("Failed to read major_version")),
		minor_version: u16::from_be_bytes(data[26..28].try_into().expect("Failed to read minor_version")),
		meta_offset: u32::from_be_bytes(data[28..32].try_into().expect("Failed to read meta_offset")),
		meta_length: u32::from_be_bytes(data[32..36].try_into().expect("Failed to read meta_length")),
		meta_orig_length: u32::from_be_bytes(data[36..40].try_into().expect("Failed to read meta_orig_length")),
		priv_offset: u32::from_be_bytes(data[40..44].try_into().expect("Failed to read priv_offset")),
		priv_length: u32::from_be_bytes(data[44..48].try_into().expect("Failed to read priv_length")),
	}
}

pub fn woff2_load_table_records(data: &[u8], num_tables: u16) -> (Vec<Woff2TableRecord>, u32, usize) {
	let mut records = Vec::new();
	let mut actual_size = 0;
	let mut offset = mem::size_of::<Woff2Header>();
	println!("Starting to read table records at offset: {}", offset);

	for _ in 0..num_tables {
		let flags: u8 = data[offset];
		println!("Flags binary: {:08b}", flags);
		offset += 1;
		let known_table_tag = flags & 0x3F;
		let transform_version = (flags >> 6) & 0x03;

		if known_table_tag > 63 {
			panic!("Invalid known_table_tag: {}", known_table_tag);
		}

		let tag = if known_table_tag == 63 {
			let tag_bytes = &data[offset..offset + 4];
			offset += 4;
			std::str::from_utf8(tag_bytes).expect("Failed to read custom tag").to_string()
		} else {
			KNOWN_TABLE_TAGS
				.get(&known_table_tag)
				.expect("Unknown table tag")
				.to_string()
		};

		let (original_length, original_length_bytes) = convert_uint_base128_to_u32(data, offset);
		offset += original_length_bytes;

		let needs_transform_length =
			matches!(tag.as_str(), "glyf" | "loca" | "htmx") || transform_version != 0;

		let transform_length = if needs_transform_length {
			println!(
				"Transform version {} detected, reading transform_length",
				transform_version
			);
			let (transform_length, transform_length_bytes) = convert_uint_base128_to_u32(data, offset);
			offset += transform_length_bytes;
			transform_length
		} else {
			0
		};

		records.push(Woff2TableRecord {
			known_table_tag,
			transform_version,
			tag,
			original_length,
			transform_length,
		});

		actual_size += original_length;
	}

	(records, actual_size, offset)
}
