struct GdefHeader {
    major_version: u16,
    minor_version: u16,
    glyph_class_def_offset: u32,
    attach_list_offset: u32,
    lig_caret_list_offset: u32,
    mark_attach_class_def_offset: u32,
    mark_glyph_sets_def_offset: u32, // Version 1.2, 1.3
    item_var_store_offset: u32, // Version 1.3
}

struct ClassDefFormat1 {
    class_format: u16,
    start_glyph_id: u16,
    glyph_count: u16,
    class_value_array: Vec<u16>,
}

struct ClassDefFormat2 {
    class_format: u16,
    class_range_count: u16,
    class_range_records: Vec<ClassRangeRecord>,
}

struct ClassRangeRecord {
    start_glyph_id: u16,
    end_glyph_id: u16,
    class_value: u16,
}
