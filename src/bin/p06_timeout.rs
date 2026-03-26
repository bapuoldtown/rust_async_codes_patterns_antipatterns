use tokio::{process, time::{Duration, sleep, timeout}};

/*
```
=== p06: timeout — Don't wait forever ===

--- Scenario 1: Slow delivery (5s), timeout at 3s ---
Delivery: on the way...
TIMEOUT! Too slow. Cooking myself.

--- Scenario 2: Fast delivery (1s), timeout at 3s ---
Delivery: on the way...
Delivery: arrived!
Got food: Pizza
```

**Key insight:**
```
 */

async fn slow_delivery() -> String{
    println!("Delivery: on the way...");
    sleep(Duration::from_secs(5)).await;
    println!("Delivery: arrived!");
    String::from("Delivery is here!")
}

async fn fast_delivery() -> String {
    println!("Delivery: on the way...");
    sleep(Duration::from_secs(1)).await;  // takes 1 second
    println!("Delivery: arrived!");
    "Pizza".to_string()
}

#[tokio::main]
async fn main() {
    let result = timeout(Duration::from_secs(3), slow_delivery()).await;
    match result{
        Ok(delivery) => println!("Received: {}", delivery),
        Err(_) => println!("Delivery took too long!"),
    }

    // SCENARIO 2: Delivery is FAST ENOUGH (1 sec < 3 sec limit)
    let result1 = timeout(Duration::from_secs(2), fast_delivery()).await;
    match result1 {
        Ok(delivery) => println!("Received: {}", delivery),
        Err(_) => println!("Delivery took too long!"),
    }
}