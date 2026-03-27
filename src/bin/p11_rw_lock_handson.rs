/*
Data changes frequently?
→ Mutex (simple, safe)

Data read 90% of time, written 10%?
→ RwLock (faster reads)
 */

 /*
 Example:
  Config settings   → RwLock (read constantly, write rarely)
  Bank balance      → Mutex  (writes happen often)
  Scoreboard        → RwLock (read often, update sometimes)
*/

use tokio::time::{sleep, Duration};
use tokio::sync::{RwLock};
use std::sync::Arc;
#[tokio::main]
async fn main(){
    println!("===Arc+RwLock====\n");
    let score = Arc::new(RwLock::new(vec!["Alice", "Bob"]));
    let s1= Arc::clone(&score);
    let s2=Arc::clone(&score);
    let s3=Arc::clone(&score);
    let t1 = tokio::spawn(async move{
        sleep(Duration::from_millis(100)).await;
        let num = s1.read().await;
        println!("[Task 1] Read score: {:?}", *num);
    });

    let t2 = tokio::spawn(async move {
        sleep(Duration::from_millis(60)).await;
        let mut vec = s2.write().await;
        
        vec.push("Charlie");
        println!("[Task 2] Updated score: {:?}", *vec);
    });

    let t3=tokio::spawn(async move{
        sleep(Duration::from_millis(2)).await;
        let vec_copy = s3.read().await;
        println!("[Task 3] Read score: {:?}", *vec_copy);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();

    let final_vec = score.read().await;
    println!("\nFinal Scoreboard: {:?}", *final_vec);
}