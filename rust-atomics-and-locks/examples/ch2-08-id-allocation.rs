use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

// This version is problematic.
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Relaxed)
}

fn main() {
    dbg!(allocate_new_id()); // 0
    dbg!(allocate_new_id()); // 1
    dbg!(allocate_new_id()); // 2

    println!("overflowing the counter... (this might take a minute)");

    let start = std::time::Instant::now();
    for _ in 3..=u32::MAX {
        allocate_new_id();
    }
    let duration = start.elapsed();
    println!("overflowed in {:?}", duration);

    dbg!(allocate_new_id()); // ⚠️ This will produce zero again. ⚠️
}
