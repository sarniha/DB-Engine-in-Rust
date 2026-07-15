mod page;
use page::*;

fn main() {
    let mut p = Page { data: [0u8; 4096] };

    // initialize header
    set_page_id(&mut p, 1);
    set_slot_count(&mut p, 0);
    set_free_space_start(&mut p, 4096);

    // insert a record
    let record = b"hello";
    let slot_id = insert_record(&mut p, record);
    println!("insert returned slot_id: {:?}", slot_id);

    let id = slot_id.unwrap();

    // confirm the record exists before deletion
    match get_record(&p, id) {
        Some(bytes) => println!("before delete: {:?}", String::from_utf8(bytes).unwrap()),
        None => println!("before delete: not found"),
    }

    // delete it
    delete_record(&mut p, id);

    // confirm it's gone now
    match get_record(&p, id) {
        Some(bytes) => println!("after delete: {:?}", String::from_utf8(bytes).unwrap()),
        None => println!("after delete: not found"),
    }
}