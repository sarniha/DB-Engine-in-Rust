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
    println!("free_space_start after insert: {}", get_free_space_start(&p));
    println!("slot_count after insert: {}", get_slot_count(&p));

    // check the slot points where we expect
    if let Some(id) = slot_id {
        println!("slot {} offset: {}", id, get_slot_offset(&p, id));
        println!("slot {} length: {}", id, get_slot_length(&p, id));
    }
    let retrieved = get_record(&p, slot_id.unwrap());
println!("retrieved: {:?}", String::from_utf8(retrieved).unwrap());
}