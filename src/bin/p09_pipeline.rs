 use tokio::sync::mpsc;
 use tokio::time::{sleep, Duration};
 async fn raju(to_priya: mpsc::Sender<i32>){
    let money =vec![100,200,300];
    for amount in money{
        println!("[Raju]  counted: {}", amount);
        sleep(Duration::from_millis(500)).await;
        to_priya.send(amount).await.unwrap();
    }
    println!("[Raju]  done!");
 }


 async fn priya(mut from_raju:mpsc::Receiver<i32>){
    while let Some(amount) = from_raju.recv().await{
        println!("[Priya]  received: {}", amount);
    }
    println!("[Priya] book complete!");
 }

 // ─────────────────────
// MAIN
// ─────────────────────
#[tokio::main]
async fn main(){
    println!("=== Two Stage Pipeline ===\n");
    // ONE channel between Raju and Priya
    let (raju_tx, priya_rx) = mpsc::channel::<i32>(5);

    // Start both simultaneously
    let r = tokio::spawn(raju(raju_tx));
    let p = tokio::spawn(priya(priya_rx));

    // Wait for both to finish
    let (r_res, p_res) = tokio::join!(r, p);

    println!("=== Done! ===");

}