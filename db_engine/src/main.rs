mod row;
mod page;
mod disk;
use row::*;
use page::*;
use disk::*;

fn main() -> Result<(), std::io::Error> {
    let path = "engine.db";

    // clean slate for repeatable testing
    let _ = std::fs::remove_file(path);

    // schema shared by both rows: (Int, Text, Float)
    let schema = Schema {
        columns: vec![ColumnType::Int, ColumnType::Text, ColumnType::Float],
    };

    // ---- Row A goes on page 0 ----
    let row_a: Row = vec![
        Value::Int(1),
        Value::Text("Alice".to_string()),
        Value::Float(92.5),
    ];
    let bytes_a = serialize_row(&row_a, &schema);

    let page_id_a = allocate_page(path)?;
    let mut page_a = Page { data: [0u8; 4096] };
    set_page_id(&mut page_a, page_id_a);
    set_slot_count(&mut page_a, 0);
    set_free_space_start(&mut page_a, 4096);
    let slot_a = insert_record(&mut page_a, &bytes_a).unwrap();
    write_page(path, page_id_a, &page_a)?;

    println!("wrote row A to page {}, slot {}", page_id_a, slot_a);

    // ---- Row B goes on the next page ----
    let row_b: Row = vec![
        Value::Int(2),
        Value::Text("Bob".to_string()),
        Value::Float(88.0),
    ];
    let bytes_b = serialize_row(&row_b, &schema);

    let page_id_b = allocate_page(path)?;
    let mut page_b = Page { data: [0u8; 4096] };
    set_page_id(&mut page_b, page_id_b);
    set_slot_count(&mut page_b, 0);
    set_free_space_start(&mut page_b, 4096);
    let slot_b = insert_record(&mut page_b, &bytes_b).unwrap();
    write_page(path, page_id_b, &page_b)?;

    println!("wrote row B to page {}, slot {}", page_id_b, slot_b);

    // ---- "Reopen" the file fresh: read both pages back from disk ----
    let loaded_page_a = read_page(path, page_id_a)?;
    let loaded_page_b = read_page(path, page_id_b)?;

    let recovered_bytes_a = get_record(&loaded_page_a, slot_a).unwrap();
    let recovered_bytes_b = get_record(&loaded_page_b, slot_b).unwrap();

    let recovered_row_a = deserialize_row(&recovered_bytes_a, &schema);
    let recovered_row_b = deserialize_row(&recovered_bytes_b, &schema);

    println!("recovered row A: {:?}", describe(&recovered_row_a));
    println!("recovered row B: {:?}", describe(&recovered_row_b));

    Ok(())
}

fn describe(row: &Row) -> String {
    let mut parts = Vec::new();
    for value in row {
        match value {
            Value::Int(i) => parts.push(format!("Int({})", i)),
            Value::Float(f) => parts.push(format!("Float({})", f)),
            Value::Text(s) => parts.push(format!("Text({})", s)),
        }
    }
    parts.join(", ")
}