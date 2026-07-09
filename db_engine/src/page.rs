pub struct Page {
    pub data: [u8; 4096],
}

pub fn get_page_id(page: &Page) -> u16 {
    u16::from_le_bytes(page.data[0..2].try_into().unwrap())
}

pub fn set_page_id(page: &mut Page, id: u16) {
    page.data[0..2].copy_from_slice(&id.to_le_bytes());
}

pub fn get_slot_count(page: &Page) -> u16 {
    u16::from_le_bytes(page.data[2..4].try_into().unwrap())
}

pub fn set_slot_count(page: &mut Page, count: u16) {
    page.data[2..4].copy_from_slice(&count.to_le_bytes());
}

pub fn get_free_space_start(page: &Page) -> u16 {
    u16::from_le_bytes(page.data[4..6].try_into().unwrap())
}

pub fn set_free_space_start(page: &mut Page, offset: u16) {
    page.data[4..6].copy_from_slice(&offset.to_le_bytes());
}