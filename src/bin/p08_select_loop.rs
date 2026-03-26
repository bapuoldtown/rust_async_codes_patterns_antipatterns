use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    println!("=== Call Centre ===\n");

    // Line 1: Complaints
    let (complaints_tx, mut complaints_rx) = mpsc::channel(5);

    // Line 2: Orders
    let (orders_tx, mut orders_rx) = mpsc::channel(5);
    // Someone sending complaints
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        complaints_tx.send("My pizza is cold!").await.unwrap();

        sleep(Duration::from_secs(2)).await;
        complaints_tx.send("Wrong order!").await.unwrap();
    });

    // Someone sending orders
    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        orders_tx.send("1x Burger").await.unwrap();

        sleep(Duration::from_secs(2)).await;
        orders_tx.send("2x Pizza").await.unwrap();
    });
    println!("Agent: ready. Watching both lines...\n");
    loop{
        tokio::select!{
            //here we match some and another explicirt None handler witha break
            Some(order) = orders_rx.recv() => println!("Agent: Received order: {}", order),
            Some(complaint) = complaints_rx.recv() => println!("Agent: Received complaint: {}", complaint),
            else => {
                println!("All lines are closed. Ending shift.");
                break;
            }
        }
    }
}