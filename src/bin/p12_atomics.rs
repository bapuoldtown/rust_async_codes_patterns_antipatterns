use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[tokio::main]
async fn main(){
    println!("=== Chai Stall Counter ===\n");
    let orders = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for customer in 1..=5{
        let counter = Arc::clone(&orders);
        let h = tokio::spawn(async move {
            let old = counter.fetch_add(1, Ordering::Relaxed);
            println!(
                "[Customer {}] before: {}  now: {}",
                customer, old, old + 1
            );
        });

        handles.push(h);
    }

    for h in handles{
        h.await.unwrap();
        
    }

    let total = orders.load(Ordering::Relaxed);
    println!("\nTotal orders: {}", total);
    println!("Expected: 5");
    println!("Correct: {}", total == 5);

}