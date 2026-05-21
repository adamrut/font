mod font_config;
// mod font_types;
// mod font_data_convert;
mod woff2;

use crate::font_config::FONT_CONFIG;
use crate::woff2::base::{woff2_load_table_records, woff2_read_header};

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