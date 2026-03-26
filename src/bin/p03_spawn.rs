use tokio::time::{sleep, Duration};

async fn boil_pasta() -> String{
    println!("Boiling pasta...");
    sleep(Duration::from_secs(3)).await;
    String::from("Pasta is ready!")
}

async fn make_chicken() -> String{
    println!("Making chicken...");
    sleep(Duration::from_secs(2)).await;
    String::from("Chicken is ready!")
}
#[tokio::main]
async fn main(){
    let join_handles_pasta = vec![tokio::spawn(boil_pasta()), tokio::spawn(make_chicken())];
    for handle in join_handles_pasta{
        match handle.await{
            Ok(result) => println!("The result is {}", result),
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }
}