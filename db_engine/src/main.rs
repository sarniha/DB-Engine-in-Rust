mod row;
mod page;
use row::*;
use page::*;

fn main() {
    // define schema: (Int, Text, Float)
    let schema = Schema {
        columns: vec![ColumnType::Int, ColumnType::Text, ColumnType::Float],
    };

    // build a row
    let original_row: Row = vec![
        Value::Int(42),
        Value::Text("Alice".to_string()),
        Value::Float(92.5),
    ];

    // serialize it
    let bytes = serialize_row(&original_row, &schema);
    println!("serialized to {} bytes", bytes.len());

    // set up a page and store the bytes
    let mut p = Page { data: [0u8; 4096] };
    set_page_id(&mut p, 1);
    set_slot_count(&mut p, 0);
    set_free_space_start(&mut p, 4096);

    let slot_id = insert_record(&mut p, &bytes).unwrap();
    println!("stored in slot {}", slot_id);

    // read it back and deserialize
    let retrieved_bytes = get_record(&p, slot_id).unwrap();
    let recovered_row = deserialize_row(&retrieved_bytes, &schema);

    // print to confirm the round trip
    for value in &recovered_row {
        match value {
            Value::Int(i) => println!("Int: {}", i),
            Value::Float(f) => println!("Float: {}", f),
            Value::Text(s) => println!("Text: {}", s),
        }
    }
}