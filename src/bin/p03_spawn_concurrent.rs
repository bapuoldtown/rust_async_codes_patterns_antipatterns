use tokio::time::{sleep, Duration, Instant};
async fn fetch_data(source: &str, latency_ms: u64) -> String {
    sleep(Duration::from_millis(latency_ms)).await;
    format!("{}: {} rows", source, latency_ms * 10)
}

#[tokio::main]
async fn main(){
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  Pattern 3: tokio::spawn — True Concurrency      ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // ── CONCURRENT with spawn ──
    println!("   🚀 Concurrent (spawn):");
     
    let start = Instant::now();
    let seq_time = start.elapsed();
    let h1 = tokio::spawn( async {fetch_data("PostgreSQL", 200).await});
    let h2 = tokio::spawn(async {fetch_data("MongoDB", 300).await});
    let h3 = tokio::spawn(async {fetch_data("Redis", 100).await});
    let r1 = h1.await.unwrap();
    let r2 = h2.await.unwrap();
    let r3 = h3.await.unwrap();
    let con_time = start.elapsed();

    println!("   {}, {}, {}", r1, r2, r3);
    println!("   Time: {:.0}ms", con_time.as_millis());
    println!("   Speedup: {:.1}x!\n", seq_time.as_millis() as f64 / con_time.as_millis() as f64);

    // ── SPAWN IN A LOOP ──
    println!("   📋 Dynamic spawn (loop):");
    let sources = vec![
        ("PostgreSQL", 200u64), ("MongoDB", 300), ("Redis", 50),
        ("Elasticsearch", 150), ("S3", 400), ("Kafka", 250),
    ];
    let start = Instant::now();
    let mut handles = Vec::new();
    for (name, time) in sources {
        let name = name.to_string();
        let handle = tokio::spawn(async move {
            fetch_data(&name, time).await
        });
        handles.push(handle);
    }

    //let mut results = Vec::new();
    //for handle in handles {
        //results.push(handle.await.unwrap());
    //}
    let loop_time = start.elapsed();

    //println!("   Results: {:?}", results);
    println!("   Time: {:.0}ms", loop_time.as_millis());
    let mut final_results = Vec::new();

    //Now lets traverse and unwrap brother
    for join_handler in handles{
        final_results.push(join_handler.await.unwrap())

    }

    println!("   Results: {:?}", final_results);




}