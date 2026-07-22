use crate::page::*;
use crate::disk::*;  // or crate::disk::* if you rename it
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

pub struct BufferPool {
    frames: HashMap<u16, Rc<RefCell<Page>>>,
    pin_counts:HashMap<u16,u32>,
    dirty_pages:HashMap<u16,bool>,
    capacity: usize,
    path: String,
    recency:VecDeque<u16>,
}

impl BufferPool {
    pub fn new(path: &str, capacity: usize) -> Self {
        BufferPool {
            frames: HashMap::new(),
            pin_counts:HashMap::new(),
            dirty_pages:HashMap::new(),
            capacity,
            path: path.to_string(),
            recency:VecDeque::new(),
        }
    }

   fn touch_recency(&mut self, page_id: u16) {
    if let Some(pos) = self.recency.iter().position(|&id| id == page_id) {
        self.recency.remove(pos);
    }
    self.recency.push_back(page_id);
}
pub fn fetch_page(&mut self, page_id: u16) -> Result<Rc<RefCell<Page>>, String> {
    if self.frames.contains_key(&page_id) {
        // cache hit
        self.touch_recency(page_id);
        *self.pin_counts.entry(page_id).or_insert(0) += 1;
        Ok(Rc::clone(&self.frames[&page_id]))
    } else {
        // cache miss
        if self.frames.len() >= self.capacity {
            self.evict_page()?;
        }
        let page = read_page(&self.path, page_id)
            .map_err(|e| format!("failed to read page {}: {}", page_id, e))?;
        let wrapped = Rc::new(RefCell::new(page));
        self.frames.insert(page_id, Rc::clone(&wrapped));
        self.pin_counts.insert(page_id, 1);
        self.dirty_pages.insert(page_id, false);
        self.touch_recency(page_id);
        Ok(wrapped)
    }
}
    pub fn unpin_page(&mut self, page_id: u16, is_dirty: bool) -> Result<(), String> {
    match self.pin_counts.get_mut(&page_id) {
        Some(count) => {
            if *count == 0 {
                return Err(format!("page {} is already unpinned", page_id));
            }
            *count -= 1;
        }
        None => {
            return Err(format!("page {} was never fetched", page_id));
        }
    }

    if is_dirty {
        self.dirty_pages.insert(page_id, true);
    }

    Ok(())
}

pub fn evict_page(&mut self) -> Result<u16, String> {
    let mut candidate: Option<u16> = None;
    for &page_id in self.recency.iter() {
        if self.pin_counts.get(&page_id) == Some(&0) {
            candidate = Some(page_id);
            break;
        }
    }
    let candidate = match candidate {
        Some(id) => id,
        None => return Err("buffer pool full: all frames pinned".to_string()),
    };
    if self.dirty_pages.get(&candidate) == Some(&true) {
        let page_ref = self.frames[&candidate].borrow();
        write_page(&self.path, candidate, &page_ref)
            .map_err(|e| format!("failed to flush page {}: {}", candidate, e))?;
        // page_ref (and its borrow) is dropped at the end of this block,
        // before we touch self.frames again below
    }
    self.frames.remove(&candidate);
    self.pin_counts.remove(&candidate);
    self.dirty_pages.remove(&candidate);
    if let Some(pos) = self.recency.iter().position(|&id| id == candidate) {
        self.recency.remove(pos);
    }
    Ok(candidate)
}
}
