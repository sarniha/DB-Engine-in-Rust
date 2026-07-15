
pub enum ColumnType {
    Int,
    Float,
    Text,
}
pub struct Schema {
    pub columns: Vec<ColumnType>,
}
pub type Row = Vec<Value>;
pub enum Value {
    Int(i32),
    Float(f64),
    Text(String),
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