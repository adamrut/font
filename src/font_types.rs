use std::collections::HashMap;
use std::sync::LazyLock;

// UInt32	signature	0x774F4632 'wOF2'
// UInt32	flavor	The "sfnt version" of the input font.
// UInt32	length	Total size of the WOFF file.
// UInt16	numTables	Number of entries in directory of font tables.
// UInt16	reserved	Reserved; set to 0.
// UInt32	totalSfntSize	Total size needed for the uncompressed font data, including the sfnt header,
//          directory, and font tables (including padding).
// UInt32	totalCompressedSize	Total length of the compressed data block.
// UInt16	majorVersion	Major version of the WOFF file.
// UInt16	minorVersion	Minor version of the WOFF file.
// UInt32	metaOffset	Offset to metadata block, from beginning of WOFF file.
// UInt32	metaLength	Length of compressed metadata block.
// UInt32	metaOrigLength	Uncompressed size of metadata block.
// UInt32	privOffset	Offset to private data block, from beginning of WOFF file.
// UInt32	privLength	Length of private data block.
#[derive(Debug)]
pub struct Woff2Header {
    pub signature: u32,
    pub flavor: u32,
    pub length: u32,
    pub num_tables: u16,
    pub reserved: u16,
    pub total_sfnt_size: u32,
    pub total_compressed_size: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub meta_offset: u32,
    pub meta_length: u32,
    pub meta_orig_length: u32,
    pub priv_offset: u32,
    pub priv_length: u32,
}


#[derive(Debug)]
pub struct Woff2TableRecord {
    pub known_table_tag: u8,
    pub transform_version: u8,
    pub tag: String,
    pub original_length: u32,
    pub transform_length: u32,
}

#[derive(Debug)]
pub struct TtfOtfHeader {
    pub sfnt_version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

#[derive(Debug)]
pub struct TtfOtfTableRecord {
    pub tag: [u8; 4],
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
}

pub static KNOWN_TABLE_TAGS: LazyLock<HashMap<u8, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(0, "cmap");
    m.insert(1, "head");
    m.insert(2, "hhea");
    m.insert(3, "hmtx");
    m.insert(4, "maxp");
    m.insert(5, "name");
    m.insert(6, "OS/2");
    m.insert(7, "post");
    m.insert(8, "cvt ");
    m.insert(9, "fpgm");
    m.insert(10, "glyf");
    m.insert(11, "loca");
    m.insert(12, "prep");
    m.insert(13, "CFF ");
    m.insert(14, "VORG");
    m.insert(15, "EBDT");
    m.insert(16, "EBLC");
    m.insert(17, "gasp");
    m.insert(18, "hdmx");
    m.insert(19, "kern");
    m.insert(20, "LTSH");
    m.insert(21, "PCLT");
    m.insert(22, "VDMX");
    m.insert(23, "vhea");
    m.insert(24, "vmtx");
    m.insert(25, "BASE");
    m.insert(26, "GDEF");
    m.insert(27, "GPOS");
    m.insert(28, "GSUB");
    m.insert(29, "EBSC");
    m.insert(30, "JSTF");
    m.insert(31, "MATH");
    m.insert(32, "CBDT");
    m.insert(33, "CBLC");
    m.insert(34, "COLR");
    m.insert(35, "CPAL");
    m.insert(36, "SVG ");
    m.insert(37, "sbix");
    m.insert(38, "acnt");
    m.insert(39, "avar");
    m.insert(40, "bdat");
    m.insert(41, "bloc");
    m.insert(42, "bsln");
    m.insert(43, "cvar");
    m.insert(44, "fdsc");
    m.insert(45, "feat");
    m.insert(46, "fmtx");
    m.insert(47, "fvar");
    m.insert(48, "gvar");
    m.insert(49, "hsty");
    m.insert(50, "just");
    m.insert(51, "lcar");
    m.insert(52, "mort");
    m.insert(53, "morx");
    m.insert(54, "opbd");
    m.insert(55, "prop");
    m.insert(56, "trak");
    m.insert(57, "Zapf");
    m.insert(58, "Silf");
    m.insert(59, "Glat");
    m.insert(60, "Gloc");
    m.insert(61, "Feat");
    m.insert(62, "Sill");
    m
});