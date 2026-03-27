use tokio::time::{sleep, Duration};
use tokio::sync::{Mutex};
use std::sync::Arc;
#[tokio::main]
async fn main(){
    println!("=== Arc + Mutex ===\n");
    // Step 1: ONE scoreboard, shared by all
    // Arc    = shared address
    // Mutex  = lock on the scoreboard
    let counter = Arc::new(Mutex::new(0));
    let s1 = Arc::clone(&counter);
    let s2 = Arc::clone(&counter);
    let s3 = Arc::clone(&counter);
    let t1 = tokio::spawn(async move{
        sleep(Duration::from_millis(100)).await;
        let mut num = s1.lock().await;
        *num +=1;

        println!("[Task 1] Incremented counter to {}", *num);

    });

    let t2 = tokio::spawn(async move{
        sleep(Duration::from_millis(300)).await;
        let mut num = s2.lock().await;
        *num+=6;
        println!("[Task 2] Incremented counter to {}", *num);
    });

    //use join with unwrap
    t1.await.unwrap();
    t2.await.unwrap();
    let final_count = counter.lock().await;
    println!("\nFinal Counter: {}", *final_count);
    
}