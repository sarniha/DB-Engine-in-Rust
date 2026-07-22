mod row;
mod page;
mod disk;
mod buffer;
use row::*;
use page::*;
use disk::*;
use buffer::*;

fn main() {
    let path = "buffer_test.db";
    let _ = std::fs::remove_file(path);

    // pre-create 5 pages on disk (0-4) so read_page succeeds
    for i in 0..5u16 {
        let mut p = Page { data: [0u8; 4096] };
        set_page_id(&mut p, i);
        write_page(path, i, &p).unwrap();
    }

    let mut pool = BufferPool::new(path, 3); // capacity 3

    let p0 = pool.fetch_page(0).unwrap();
    let p1 = pool.fetch_page(1).unwrap();
    let p2 = pool.fetch_page(2).unwrap();
    // pool now full (3/3), all pinned

    // modify page 0 in memory, mark dirty on unpin
    p0.borrow_mut().data[10] = 77;
    pool.unpin_page(0, true).unwrap();
    pool.unpin_page(1, false).unwrap();
    pool.unpin_page(2, false).unwrap(); // 2 stays pinned? no - unpin all so eviction can pick any

    // fetch page 3 — pool full, must evict. Page 0 is LRU among unpinned (0,1,2 all unpinned now, 0 fetched first = oldest)
    let _p3 = pool.fetch_page(3).unwrap();

    // confirm page 0 was flushed to disk with our modification BEFORE eviction
    let reloaded = read_page(path, 0).unwrap();
    println!("page 0 byte[10] after eviction: {}", reloaded.data[10]); // expect 77

    println!("test complete");
}