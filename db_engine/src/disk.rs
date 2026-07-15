use crate::page::*;
use std::io::{Seek,SeekFrom,Write,Read};
use std::fs::OpenOptions;

pub fn write_page(path: &str,page_id: u16,page :&Page)->Result<(), std::io::Error>

{
    let mut file=OpenOptions::new().write(true).create(true).open(path)?;
    let offset=page_id as u64*4096;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&page.data)?;
    Ok(())

}

pub fn read_page(path: &str,page_id: u16)->Result<Page,std::io::Error>{
    let mut file=OpenOptions::new().read(true).open(path)?;
    let offset=page_id as u64*4096;
    file.seek(SeekFrom::Start(offset))?;
    let mut buffer: [u8; 4096] = [0u8; 4096];
    file.read_exact(&mut buffer)?;
    Ok(Page{data:buffer})

}
pub fn allocate_page(path: &str)-> Result<u16, std::io::Error>{
    let  file=OpenOptions::new().read(true).write(true).create(true).open(path)?;
    let size=file.metadata()?.len();
    Ok((size/4096) as u16)
}