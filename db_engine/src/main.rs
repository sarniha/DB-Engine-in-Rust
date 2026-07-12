mod page;
use page::*;

fn main() {
    let mut p = Page { data: [0u8; 4096] };
    set_page_id(&mut p, 42);
    set_slot_count(&mut p, 0);
    set_free_space_start(&mut p, 4096);
    set_slot_offset(&mut p,2,26);

    println!("page_id: {}", get_page_id(&p));
    println!("slot_count: {}", get_slot_count(&p));
    println!("free_space_start: {}", get_free_space_start(&p));
    println!("slot{}",get_slot_offset(&p,2));
    set_slot_length(&mut p, 2, 50);
println!("slot 2 length: {}", get_slot_length(&p, 2));
}