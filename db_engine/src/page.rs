pub struct Page {
    pub data: [u8; 4096],
}

//Header functions

pub fn get_page_id(page: &Page) -> u16 {
    u16::from_le_bytes(page.data[0..2].try_into().unwrap())
}

pub fn set_page_id(page: &mut Page, id: u16) {
    page.data[0..2].copy_from_slice(&id.to_le_bytes());
}
//page.data gets copied data brought by to_le_bytes

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

pub fn get_slot_offset(page: &Page, slot_id: u16) -> u16 {
    let start = 6 + (slot_id * 4) as usize;
    u16::from_le_bytes(page.data[start..start+2].try_into().unwrap())
}

pub fn set_slot_offset(page: &mut Page,slot_id:u16,offset: u16){
    let start=6+(slot_id*4) as usize;
    page.data[start..start+2].copy_from_slice(&offset.to_le_bytes());


}
pub fn set_slot_length(page: &mut Page,slot_id:u16,length: u16){
    let start=6+(slot_id*4) as usize;
    page.data[start+2..start+4].copy_from_slice(&length.to_le_bytes());
}
pub fn get_slot_length(page: &Page, slot_id: u16) -> u16 {
    let start = 6 + (slot_id * 4) as usize;
    u16::from_le_bytes(page.data[start+2..start+4].try_into().unwrap())
}