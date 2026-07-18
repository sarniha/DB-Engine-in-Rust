use crate::page::*;
use crate::disk::*;  // or crate::disk::* if you rename it
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
pub struct BufferPool {
    frames: HashMap<u16, Rc<RefCell<Page>>>,
    capacity: usize,
    path: String,
}

impl BufferPool {
    pub fn new(path: &str, capacity: usize) -> Self {
        BufferPool {
            frames: HashMap::new(),
            capacity,
            path: path.to_string(),
        }
    }

    pub fn fetch_page(&mut self, page_id: u16) -> Rc<RefCell<Page>> {
        // cache hit
        if let Some(page_rc) = self.frames.get(&page_id) {
            return Rc::clone(page_rc);
        }

        // cache miss — check capacity (placeholder, real eviction comes in Step 5)
        if self.frames.len() >= self.capacity {
            panic!("Buffer pool full — eviction not yet implemented (TODO Step 5)");
        }

        // load from disk
        let page = read_page(&self.path, page_id)
            .expect("failed to read page from disk");

        let wrapped = Rc::new(RefCell::new(page));
        self.frames.insert(page_id, Rc::clone(&wrapped));
        wrapped
    }
}