mod font_config;
mod font_types;
mod font_data_convert;

use crate::font_config::FONT_CONFIG;
use crate::font_types::Woff2Header;
use crate::font_types::KNOWN_TABLE_TAGS;
use crate::font_data_convert::convert_uint_base128_to_u32;

use std::mem;
// use std::fs::File;
// use std::io::{Read, Seek, SeekFrom, Cursor};

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

    if signature == 0x774F4632 {
        println!("Font is in WOFF2 format");
        let header = woff2_read_header(&font_data);
        println!("WOFF2 Header: {:?}", header);
        let table_records = woff2_load_table_records(&font_data, header.num_tables);
        println!("WOFF2 Table Records: {:?}", table_records);
    } else if signature == 0x00010000 || signature == 0x74727565 {
        println!("Font is in TTF format");
    } else if signature == 0x4F54544F {
        println!("Font is in OTF format");
    } else {
        println!("Unknown font format");
    }


    // let mut font_file = File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
    //     .expect("Failed to open font file");

    // let mut buffer = [0u8; 12];
    // font_file.read_exact(&mut buffer).expect("Failed to read from font file");
    // println!("Buffer bytes: {:02X?}", buffer);

    // let mut reader = Cursor::new(buffer);
    // reader.read_exact(&mut buffer).expect("Failed to read from cursor");

    // let sfnt_version: u32 = u32::from_be_bytes(buffer[0..4].try_into()
    //     .expect("Failed to convert to u32"));

    // let num_tables: u16 = u16::from_be_bytes(buffer[4..6].try_into()
    //     .expect("Failed to convert to u16"));

    // let search_range: u16 = u16::from_be_bytes(buffer[6..8].try_into()
    //     .expect("Failed to convert to u16"));

    // let entry_selector: u16 = u16::from_be_bytes(buffer[8..10].try_into()
    //     .expect("Failed to convert to u16"));

    // let range_shift: u16 = u16::from_be_bytes(buffer[10..12].try_into()
    //     .expect("Failed to convert to u16"));

    // println!("SFNT Version: {:08X}", sfnt_version);
    // println!("Number of Tables: {}", num_tables);
    // println!("Search Range: {}", search_range);
    // println!("Entry Selector: {}", entry_selector);
    // println!("Range Shift: {}", range_shift);
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

fn woff2_load_table_records(data: &[u8], num_tables: u16) -> Vec<font_types::Woff2TableRecord> {
    let mut records = Vec::new();
    let mut offset = mem::size_of::<Woff2Header>(); // Start after the header
    println!("Starting to read table records at offset: {}", offset);

    for _ in 0..num_tables {
        // Read the flags byte to determine the known_table_tag and transform_version
        let flags = data[offset];
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
        let (original_length, original_length_bytes) = convert_uint_base128_to_u32(data, offset + 5);
        let (transform_length, transform_length_bytes) = convert_uint_base128_to_u32(data, offset + 5 + original_length_bytes);

        records.push(font_types::Woff2TableRecord {
            known_table_tag,
            transform_version,
            tag,
            original_length,
            transform_length,
        });

        offset += original_length_bytes + transform_length_bytes; // Move to the next record
    }

    records
}