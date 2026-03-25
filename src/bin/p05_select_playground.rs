use tokio::time::{sleep, Duration};

async fn fetch_from_india() -> String {
    sleep(Duration::from_millis(100)).await;
    "Data from India server".to_string()
}

async fn fetch_from_usa() -> String {
    sleep(Duration::from_millis(300)).await;
    "Data from USA server".to_string()
}

#[tokio::main]
async fn main(){
    //The Simplest Possible select!
    tokio::select!{
        _ = sleep(Duration::from_secs(6)) =>{
            println!("6s timer fired FIRST!");
        },
        _ = sleep(Duration::from_secs(10)) =>{
            println!("10s timer expired second");

        }
    };

    tokio::select!{
        data1 = fetch_from_india() => {
            println!("India server responded FIRST! {}", data1);
        },
        data2 = fetch_from_usa() => {
            println!("USA server responded FIRST! {}", data2);
        }

    }


}