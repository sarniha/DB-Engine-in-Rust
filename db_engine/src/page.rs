pub struct Page {
    pub data: [u8; 4096],
}
pub enum Value {
    Int(i32),
    Float(f64),
    Text(String),
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
pub fn get_record(page: &Page, slot_id: u16) ->Option< Vec<u8>> {
    let offset = get_slot_offset(page, slot_id) as usize;
    let length = get_slot_length(page, slot_id) as usize;
    if length==0{
        return None;
    }
    Some(page.data[offset..offset + length].to_vec())
}
pub fn delete_record(page: &mut Page,slot_id:u16){
    set_slot_length(page,slot_id,0);
}



pub fn serialize_row(row: &Row, schema: &Schema) -> Vec<u8> {
    let mut output = Vec::new();

    for (value, col_type) in row.iter().zip(schema.columns.iter()) {
        match value {
            Value::Int(i) => {
                output.extend_from_slice(&i.to_le_bytes());
            }
            Value::Float(f) => {
                output.extend_from_slice(&f.to_le_bytes());
            }
            Value::Text(s) => {
                let len = s.len() as u16;
                output.extend_from_slice(&len.to_le_bytes());
                output.extend_from_slice(s.as_bytes());
            }
        }
    }

    output
}

pub fn deserialize_row(bytes: &[u8], schema: &Schema) -> Row{
    let mut row:Row=Vec::new();
    let mut position: usize=0;
    for col_type in schema.columns.iter(){
        match col_type{
            ColumnType::Int=>{
                let value=i32::from_le_bytes(bytes[position..position+4].try_into().unwrap());
                row.push(Value::Int(value));
                position+=4;
            }
            ColumnType::Float=>{
                let value=f64::from_le_bytes(bytes[position..position+8].try_into().unwrap());
                row.push(Value::Float(value));
                position+=8;
            }
            ColumnType::Text=>{
                let length=u16::from_le_bytes(bytes[position..position+2].try_into().unwrap());
                let start=position+2;
                let end=start+length as usize;
                let value=String::from_utf8(bytes[start..end].to_vec()).unwrap();
                row.push(Value::Text(value));
                position=end;
            }
        }
    }
    row
}