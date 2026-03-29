/*
GOOD CODE (same order):
─────────────────────────────
Arjun: knife → fork
Priya: knife → fork  ← SAME!

What happens:
  Arjun grabs knife 🔪  (gets it!)
  Priya tries knife  🔪  (Arjun has it → WAITS)
  
  Arjun grabs fork 🍴   (gets it! nobody competing)
  Arjun eats 🍽️
  Arjun drops fork 🍴
  Arjun drops knife 🔪
  
  Priya grabs knife 🔪  (now free!)
  Priya grabs fork 🍴   (now free!)
  Priya eats 🍽️
  
  NO CIRCLE = NO DEADLOCK ✅
 */

use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== NO DEADLOCK — safe ===\n");

    let knife = Arc::new(Mutex::new("knife"));
    let fork  = Arc::new(Mutex::new("fork"));

    let knife1 = Arc::clone(&knife);
    let fork1  = Arc::clone(&fork);

    // Arjun: knife THEN fork
    let t1 = tokio::spawn(async move {
        let k = knife1.lock().await;  // knife first
        println!("[Arjun] has knife!");
        sleep(Duration::from_millis(100)).await;
        let f = fork1.lock().await;   // fork second
        println!("[Arjun] has fork! Eating! 🍽️");
        drop(f); drop(k);
    });

    let knife2 = Arc::clone(&knife);
    let fork2  = Arc::clone(&fork);

    // Priya: ALSO knife THEN fork (same order!)
    let t2 = tokio::spawn(async move {
        let k = knife2.lock().await;  // knife first
        println!("[Priya] has knife!");
        sleep(Duration::from_millis(100)).await;
        let f = fork2.lock().await;   // fork second
        println!("[Priya] has fork! Eating! 🍽️");
        drop(f); drop(k);
    });

    tokio::join!(t1, t2);
    println!("\n✅ Both ate! No deadlock!");
}