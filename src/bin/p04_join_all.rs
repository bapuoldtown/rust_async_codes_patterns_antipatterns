use tokio::time::{sleep, Duration, Instant};
async fn fetch_data(source: &str, latency_ms: u64) -> String {
    sleep(Duration::from_millis(latency_ms)).await;
    println!("   ✅ [{}] done ({}ms)", source, latency_ms);
    format!("{}: {} rows", source, latency_ms * 10)
}

async fn fallible_fetch(source: &str, latency_ms: u64, fail: bool) -> Result<String, String> {
    sleep(Duration::from_millis(latency_ms)).await;
    if fail { Err(format!("{} failed!", source)) }
    else { Ok(format!("{}: success", source)) }
}
#[tokio::main]
async fn main(){
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  Pattern 4: join! — Run ALL, Wait For ALL        ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // ── Basic join! ──
    println!("━━━ Concurrent Fetches ━━━\n");
    let start = Instant::now();
    let (pg, mongo, redis) = tokio::join!(
        fetch_data("PostgreSQL", 200),
        fetch_data("MongoDB", 300),
        fetch_data("Redis", 50),
    );
    println!("\n   Total: {:.0}ms (sequential would be ~550ms)\n", start.elapsed().as_millis());

    // ── try_join! ──
    println!("━━━ try_join! — Fail Fast ━━━\n");
    println!("   All succeed:");

    let output = tokio::try_join!(
        fallible_fetch("A", 200, false),
        fallible_fetch("B", 300, false),
    );
    match output{
        Ok((a, b)) => println!("   ✅ {}, {}", a, b),
        Err(e) => println!("   ❌ {}", e),
    }

    println!("   One fails:");
    match tokio::try_join!(
        fallible_fetch("A", 200, false),
        fallible_fetch("B", 100, true),   // FAILS at 100ms!
        fallible_fetch("C", 300, false),
    ) {
        Ok((a, b, c)) => println!("   ✅ {}, {}, {}", a, b, c),
        Err(e) => println!("   ❌ {} (stopped early, didn't wait for A or C!)", e),
    }

    println!("\n   Key: join! = concurrent + wait ALL, try_join! = stop at first error");


}