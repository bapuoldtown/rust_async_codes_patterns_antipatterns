use tokio::pin;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    //Create a channel with buffer of size 20
    // Step 1: Create the window
    // tx = kitchen side (sender)
    // rx = counter side (receiver)
    let (tx, mut rx) = mpsc::channel(20);

    // Step 2: Spawn the KITCHEN task
    // Kitchen makes burgers and pushes through window

    tokio::spawn(async move {
        let burgers = vec!["BigMac", "McChicken", "Filet-O-Fish"];
        for burger in burgers{
            println!("Kitchen: Cooking {}...", burger);
            sleep(Duration::from_secs(5)).await; // Simulate cooking time
            // Push through window
            match tx.send(burger).await{
                Ok(j) => println!("Kitchen: Sent {:?} to counter", j),
                Err(e) => println!("Kitchen: Failed to send {}. Error: {:?}", burger, e),
            }


        }
        println!("Kitchen: closing window.");
        // tx is dropped here → window closes
    });

    while let Some(order) = rx.recv().await{
        println!("Counter: Received order for {}", order);
        println!("Counter: Serving {}", order);
    }

}