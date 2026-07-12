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
pub fn insert_record(page: &mut Page, record: &[u8]) -> Option<u16>{
    let slot_id=get_slot_count(page);
    let slot_end=6+(slot_id as usize *4)+4 ;
    let record_start=get_free_space_start(page)as usize-record.len();
    if slot_end>=record_start{
        return None;
    }


    else{

    
    page.data[record_start..record_start+record.len()].copy_from_slice(record);
    set_slot_offset( page,slot_id,record_start as u16);
    set_slot_length( page,slot_id,record.len() as u16);
    
    set_slot_count( page,slot_id+1 as u16);
    set_free_space_start(page, record_start as u16);
    Some(slot_id)
    }



}
pub fn get_record(page: &Page, slot_id: u16) -> Vec<u8> {
    let offset = get_slot_offset(page, slot_id) as usize;
    let length = get_slot_length(page, slot_id) as usize;
    page.data[offset..offset + length].to_vec()
}