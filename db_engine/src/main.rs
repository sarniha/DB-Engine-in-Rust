mod row;
mod page;
mod disk;
use row::*;
use page::*;
use disk::*;

fn stress_test_insert_until_full() {
    let mut p = Page { data: [0u8; 4096] };
    set_page_id(&mut p, 1);
    set_slot_count(&mut p, 0);
    set_free_space_start(&mut p, 4096);

    let mut inserted_count = 0;

    for i in 0..2000u16 {
        let record = vec![i as u8; 10]; // 10 bytes, all holding value i
        match insert_record(&mut p, &record) {
            Some(slot_id) => {
                inserted_count += 1;
                // sanity check slot_id matches insertion order
                assert_eq!(slot_id, i, "slot_id out of order!");
            }
            None => {
                println!("Page full after {} records", inserted_count);
                break;
            }
        }
    }

    // now verify every inserted record is still intact, uncorrupted
    for i in 0..inserted_count {
        let expected = vec![i as u8; 10];
        match get_record(&p, i) {
            Some(actual) => {
                assert_eq!(actual, expected, "Record {} corrupted!", i);
            }
            None => {
                panic!("Record {} missing but should exist!", i);
            }
        }
    }

    println!("Stress test passed: {} records verified intact", inserted_count);
}

fn main() {
    stress_test_insert_until_full();
}