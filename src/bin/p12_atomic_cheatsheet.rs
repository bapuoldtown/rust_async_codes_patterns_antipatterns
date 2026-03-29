use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
fn main(){
    let counter  = AtomicU64::new(0);
    let is_ready = AtomicBool::new(false);
    // store
    counter.store(10, Ordering::Relaxed);

    // load
    println!("value: {}", counter.load(Ordering::Relaxed)); // 10
     // fetch_add
    let old = counter.fetch_add(5, Ordering::Relaxed);
    println!("old: {}, now: {}", old, 
        counter.load(Ordering::Relaxed)); // 10, 15
    
    // fetch_sub
    counter.fetch_sub(3, Ordering::Relaxed);
    println!("after sub: {}", 
        counter.load(Ordering::Relaxed)); // 12
    
    // fetch_max
    counter.fetch_max(100, Ordering::Relaxed);
    println!("after max: {}", 
        counter.load(Ordering::Relaxed)); // 100\
    
    // fetch_min
    counter.fetch_min(50, Ordering::Relaxed);
    println!("after min: {}", 
        counter.load(Ordering::Relaxed)); // 50
    
    // swap
    let old = counter.swap(999, Ordering::Relaxed);
    println!("swapped: {} → 999", old); // 50 → 999

    // AtomicBool
    is_ready.store(true, Ordering::Relaxed);
    println!("ready: {}", 
        is_ready.load(Ordering::Relaxed)); // true
}