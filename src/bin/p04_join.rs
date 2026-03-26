//The story: Same kitchen. But now you're a waiter. You don't serve the table until BOTH pasta AND chicken are ready. You want ALL dishes together.
//join!  → ALL must finish → then continue
//spawn  → fire and forget → continue immediately
use tokio::time::{sleep, Duration};
async fn make_pasta() -> String{
    println!("Pasta: cooking...");
    sleep(Duration::from_secs(3)).await;
    println!("Pasta: ready!");
    String::from("Pasta is ready")
}

async fn make_chicken() -> String{
    println!("Chicken: cooking...");
    sleep(Duration::from_secs(2)).await;
    println!("Chicken is ready");
    String::from("Chicken is ready")

}

async fn make_salad() -> String {
    println!("Salad: preparing...");
    sleep(Duration::from_secs(1)).await;
    println!("Salad: ready!");
    "Salad".to_string()
}

#[tokio::main]
async fn main() {
    //join! automatically runs the await and waits untill all functions have returned the results
    let (a,b,c) = tokio::join!(make_pasta(), make_chicken(), make_salad());
    println!("All dishes are ready: {}, {}, {}", a, b, c);
}