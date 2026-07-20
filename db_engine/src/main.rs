mod row;
mod page;
mod disk;
mod buffer;
use row::*;
use page::*;
use disk::*;
use buffer::*;

fn main() {
    let mut pool = BufferPool::new("engine.db", 3);

    let page1 = pool.fetch_page(0);
    let page2 = pool.fetch_page(0); // same page_id again — should be cache hit

    println!("Fetched page 0 twice, pool now has {} frames", 1); // we'll verify hit/miss properly next
}