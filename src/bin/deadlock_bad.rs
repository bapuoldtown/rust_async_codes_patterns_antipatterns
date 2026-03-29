/*
BAD CODE (different order):
─────────────────────────────
Arjun: knife → fork
Priya: fork  → knife

What happens:
  Arjun grabs knife 🔪
  Priya grabs fork  🍴
  
  Arjun needs fork  → Priya has it → WAIT
  Priya needs knife → Arjun has it → WAIT
  
  CIRCLE! Nobody moves. 💀
  
  knife ←── Arjun ──→ needs fork
    ↑                      ↓
  Priya ←── needs ←── has fork
  
  Circular dependency = DEADLOCK
 */

use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== DEADLOCK — watch it freeze ===\n");

    let knife = Arc::new(Mutex::new("knife"));
    let fork  = Arc::new(Mutex::new("fork"));

    let knife1 = Arc::clone(&knife);
    let fork1  = Arc::clone(&fork);

    // Arjun: grabs knife THEN wants fork
    let t1 = tokio::spawn(async move {
        let k = knife1.lock().await;
        println!("[Arjun] has knife, wants fork...");
        sleep(Duration::from_millis(100)).await;
        let f = fork1.lock().await;  // WAITS for fork
        println!("[Arjun] eating!"); // never reaches here
        drop(f); drop(k);
    });

    let knife2 = Arc::clone(&knife);
    let fork2  = Arc::clone(&fork);

    // Priya: grabs fork THEN wants knife
    let t2 = tokio::spawn(async move {
        let f = fork2.lock().await;
        println!("[Priya] has fork, wants knife...");
        sleep(Duration::from_millis(100)).await;
        let k = knife2.lock().await; // WAITS for knife
        println!("[Priya] eating!"); // never reaches here
        drop(k); drop(f);
    });

    tokio::join!(t1, t2);
    println!("Done!"); // never reaches here either 💀
}