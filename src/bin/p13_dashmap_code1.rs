/*
DashMap = HashMap that multiple tasks 
          can use SIMULTANEOUSLY
          No manual locking needed
 */

 /*
 Mutex<HashMap>:
  Task 1 reads key "Arjun" → LOCKS entire map
  Task 2 reads key "Priya" → WAITS 😩
  Even different keys = waiting!

DashMap:
  Task 1 reads "Arjun" → no wait
  Task 2 reads "Priya" → no wait
  Different keys = simultaneous ✅
  */

use dashmap::DashMap;
use std::sync::Arc;

#[tokio::main]
async fn main(){
    println!("=== DashMap Demo ===\n");
    let scores: Arc<DashMap<String, u32>> = Arc::new(DashMap::new());

    // Insert
    scores.insert("Arjun".to_string(), 100);
    scores.insert("Priya".to_string(), 200);
    scores.insert("Rahul".to_string(), 150);

    let s1 = Arc::clone(&scores);
    let s2 = Arc::clone(&scores);
    let s3 = Arc::clone(&scores);

    let t1 = tokio::spawn(async move{
        if let Some(mut val) = s1.get_mut("Arjun"){
            *val += 50;
            println!("[Task 1] Arjun score: {}", *val);
        }
    });

    let t2 = tokio::spawn(async move {
        // update Priya
        if let Some(mut val) = s2.get_mut("Priya") {
            *val += 50;
            println!("[Task 2] Priya score: {}", *val);
        }
    });

    let t3 = tokio::spawn(async move {
        // read Rahul
        if let Some(val) = s3.get("Rahul") {
            println!("[Task 3] Rahul score: {}", *val);
        }
    });

    tokio::join!(t1, t2, t3);

    println!("\nAll scores: {:?}", scores);
    for entry in scores.iter() {
        println!("  {}: {}", entry.key(), entry.value());
    }

}