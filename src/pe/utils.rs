use super::section_table;

pub fn is_in_range (rva: usize, r1: usize, r2: usize) -> bool {
    r1 <= rva && rva < r2
}

fn rva2offset (rva: usize, section: &section_table::SectionTable) -> usize {
    (rva - section.virtual_address as usize) + section.pointer_to_raw_data as usize
}

fn is_in_section (rva: usize, section: &section_table::SectionTable) -> bool {
    section.virtual_address as usize <= rva && rva < (section.virtual_address + section.virtual_size) as usize
}

pub fn find_offset (rva: usize, sections: &[section_table::SectionTable]) -> Option<usize> {
    for section in sections {
        if is_in_section(rva, &section) {
            return Some(rva2offset(rva, &section))
        }
    }
    None
}
