
pub fn convert_uint_base128_to_u32(data: &[u8], offset: usize) -> (u32, usize) {
    let mut value: u32 = 0;
    let mut bytes_read = 0;

    for i in 0..5 {
        let byte = data[offset + i];
        // println!("Reading Base128 Byte {}: {:02X}", i, byte);
        value = (value << 7) | (byte as u32 & 0x7F);
        // println!("Intermediate Value after Byte {}: {:08X}", i, value);
        bytes_read += 1;

        if byte & 0x80 == 0 {
            break;
        }
    }

    (value, bytes_read)
}
