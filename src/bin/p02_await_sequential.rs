/*
Pattern 2: .await Is SEQUENTIAL — One After Another
Run with: cargo run --bin p02_await_sequential
The Big Idea
.await suspends the current task until the future completes.
Multiple .awaits in a row execute ONE AT A TIME, in order.
The thread is free during each .await, but THIS task waits.
Real-Life Analogy
You order pizza. You STAND AT THE DOOR waiting for delivery (30 min).
Pizza arrives. NOW you order sushi. Wait at door again (45 min).
Sushi arrives. NOW you order cake. Wait (20 min).
Total: 95 minutes of door-waiting!
Smart approach: Order all three at once! (That's join! in Pattern 4)
When Sequential IS Correct
When step 2 DEPENDS on step 1's result!

Can't cook pasta before water boils
Can't search database without the query embedding
Can't transform data you haven't fetched yet
 */
 use tokio::time::{sleep, Duration, Instant};
 async fn fetch_data(source: &str, latency_ms: u64) ->Result<String, Box<dyn std::error::Error>> {
    println!("   📡 [{}] Request sent...", source);
    sleep(Duration::from_millis(latency_ms)).await;
    let result = format!("{}: {} rows fetched", source, latency_ms * 10);
    Ok(result)
    
 }

 async fn fetch_data1(source: &str, latency_ms: u64) -> String {
    println!("   📡 [{}] Request sent...", source);
    sleep(Duration::from_millis(latency_ms)).await;
    let result = format!("{}: {} rows fetched", source, latency_ms * 10);
    result
    
 }

 /// Simulates a step that DEPENDS on a previous step's output

 async fn transform_data(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("   🔄 Transforming: '{}'...", input);
    sleep(Duration::from_millis(100)).await;
    let output = format!("transformed({})", input);
    println!("   ✅ Transform complete");
    Ok(output)
 }

 async fn transform_data1(input: &str) -> String {
    println!("   🔄 Transforming: '{}'...", input);
    sleep(Duration::from_millis(100)).await;
    let output = format!("transformed({})", input);
    println!("   ✅ Transform complete");
    output
 }

 #[tokio::main]
 async fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  Pattern 2: .await Is Sequential                 ║");
    println!("╚══════════════════════════════════════════════════╝\n");
    // ══════════════════════════════════════════════
    // EXPERIMENT 1: Three independent fetches done sequentially
    // ══════════════════════════════════════════════
    println!("━━━ Experiment 1: Sequential Fetches (SLOW!) ━━━\n");
    println!("   These 3 fetches are INDEPENDENT — they don't need");
    println!("   each other's results. But .await forces them in order.\n");

    let start = Instant::now();
    //  Timeline:
    //  ──[Postgres 200ms]──[Redis 100ms]──[S3 300ms]──
    //  Total: 200 + 100 + 300 = 600ms  (sum of all waits!)
    
    
    match fetch_data("PostgreSQL", 200).await{
        Ok(r1)=>{
            match transform_data(&r1).await{
                Ok(r2)=> println!("The final returned value is {:?}", r2),
                Err(e) => eprintln!("Thread panicked with error: {:?}", e),
            };

        },
        Err(err) => {
            eprintln!("Thread panicked with error: {:?}", err)

        }
    };

    let elapsed = start.elapsed();
    println!("   ⏱️  Total: {:.0}ms (sequential is CORRECT here)\n", elapsed.as_millis());
    

    //let r1=fetch_data1("PostgreSQL", 200).await;
    //println!("The data fetched is {:?}", r1);
    //let r2 = transform_data1(&r1).await;
    //println!("The data fetched is {:?}", r2);

    // ══════════════════════════════════════════════
    // EXPERIMENT 3: Thread is FREE during .await
    // ══════════════════════════════════════════════
    println!("━━━ Experiment 3: The Thread Is FREE During .await ━━━\n");
    let start = Instant::now();
    tokio::spawn(async move {
        for i in 1..=5{
            sleep(Duration::from_millis(100)).await;
            println!("   🏃 [Background] Tick {} at {:.0}ms — thread NOT blocked!",
                i, start.elapsed().as_millis());
        }
    });
    println!("   ⏳ [Main] Starting a 500ms fetch...");
    match fetch_data("REDISL", 200).await{
        Ok(r1)=>{println!("The data fetched is {:?}", r1);},
        Err(e) =>{println!("The error panicked is {:?}", e);},
    }


 }