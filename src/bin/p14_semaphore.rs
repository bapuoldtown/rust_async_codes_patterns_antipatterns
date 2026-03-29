use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tokio::sync::Semaphore;

#[tokio::main]
async fn main(){
    // Only 2 allowed at same time
    let toilet = Arc::new(Semaphore::new(2));

    let t1 = Arc::clone(&toilet);
    let t2 = Arc::clone(&toilet);
    let t3 = Arc::clone(&toilet);

    let h1 = tokio::spawn(async move {
        let _enter = t1.acquire().await.unwrap();
        println!("Person 1: inside");
        sleep(Duration::from_secs(30)).await;
        println!("Person 1: leaving");
    });

    // Person 2
    let h2 = tokio::spawn(async move {
        let _enter = t2.acquire().await.unwrap(); // enter
        println!("Person 2: inside");
        sleep(Duration::from_secs(20)).await;
        println!("Person 2: leaving");
    });

    // Person 3 — has to WAIT (toilet full!)
    let h3 = tokio::spawn(async move {
        println!("Person 3: WAITING outside...");
        let _enter = t3.acquire().await.unwrap(); // waits!
        println!("Person 3: inside finally!");
        sleep(Duration::from_secs(15)).await;
        println!("Person 3: leaving");
    });

    tokio::join!(h1, h2, h3);




}