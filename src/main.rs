mod font_config;
mod font_types;
mod font_data_convert;

use crate::font_config::FONT_CONFIG;
use crate::font_types::Woff2Header;
use crate::font_types::KNOWN_TABLE_TAGS;
use crate::font_data_convert::convert_uint_base128_to_u32;

use std::mem;
use std::io::Read;

use reqwest::blocking::Client;


fn main() {

    println!("Little endian: {}", FONT_CONFIG.is_little_endian);

    let request_client = Client::new();

    let font_url = "https://fonts.gstatic.com/s/inter/v20/UcCo3FwrK3iLTcviYwYZ90A2N58.woff2";

    let font_data = request_client.get(font_url)
        .send()
        .expect("Failed to fetch font data")
        .bytes()
        .expect("Failed to read font data");
    
    let signature = u32::from_be_bytes(font_data[0..4].try_into()
        .expect("Failed to convert to u32"));

    // WOFF2
    if signature == 0x774F4632 { 
        println!("Font is in WOFF2 format");
        let header = woff2_read_header(&font_data);
        println!("WOFF2 Header: {:#?}", header);
        let (table_records, actual_size, offset) = woff2_load_table_records(&font_data, header.num_tables);
        println!("WOFF2 Table Records: {:#?}", table_records);
        println!("Actual size of uncompressed data: {}", actual_size);
        println!("Offset after reading table records: {}", offset);
        println!("End of compressed data: {}", offset + header.total_compressed_size as usize);

        let compressed_end = offset + header.total_compressed_size as usize;
        let mut decompressor = brotli::Decompressor::new(&font_data[offset..compressed_end], header.total_sfnt_size as usize);
        let mut table_data = Vec::new();
        decompressor.read_to_end(&mut table_data).expect("Failed to decompress table data");
        println!("Decompressed table data length: {}", table_data.len());
    // TrueType
    } else if signature == 0x00010000 || signature == 0x74727565 {
        println!("Font is in TTF format");
    // OTF
    } else if signature == 0x4F54544F {
        println!("Font is in OTF format");
    } else {
        println!("Unknown font format");
    }
}

fn woff2_read_header(data: &[u8]) -> Woff2Header {
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

fn woff2_load_table_records(data: &[u8], num_tables: u16) -> (Vec<font_types::Woff2TableRecord>, u32, usize) {
    let mut records = Vec::new();
    let mut actual_size = 0;
    let mut offset = mem::size_of::<Woff2Header>(); // Start after the header
    println!("Starting to read table records at offset: {}", offset);

    for _ in 0..num_tables {
        // Read the flags byte to determine the known_table_tag and transform_version
        let flags: u8 = data[offset];
        println!("Flags binary: {:08b}", flags);
        offset += 1; // Move past the flags byte
        let known_table_tag = flags & 0x3F;
        let transform_version = (flags >> 6) & 0x03;

        // Woff2 table tags only go up to 63
        if known_table_tag > 63 {
            panic!("Invalid known_table_tag: {}", known_table_tag);
        }

        // Determine the tag string based on the known_table_tag
        let tag = if known_table_tag == 63 {
            let tag_bytes = &data[offset..offset + 4];
            offset += 4; // Move past the custom tag bytes
            std::str::from_utf8(tag_bytes).expect("Failed to read custom tag").to_string()
        } else {
            KNOWN_TABLE_TAGS.get(&known_table_tag)
                .expect("Unknown table tag")
                .to_string()
        };

        // Read the original_length and transform_length using Base128 encoding
        let (original_length, original_length_bytes) = convert_uint_base128_to_u32(data, offset);
        offset += original_length_bytes; // Move past the original_length bytes

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
    

        records.push(font_types::Woff2TableRecord {
            known_table_tag,
            transform_version,
            tag,
            original_length,
            transform_length,
        });

        actual_size += original_length; // Accumulate the actual size of the uncompressed data
    }

    (records, actual_size, offset)
}