/*
Pattern 1: Futures Are LAZY — Nothing Runs Until .await
Run with: cargo run --bin p01_lazy_futures
The Big Idea
In Rust, calling an async function does NOT execute it.
It creates a Future object — a "plan" sitting in memory.
Only .await (or the runtime polling it) actually runs the code.
This is DIFFERENT from JavaScript where async functions run immediately!
Real-Life Analogy

Writing a recipe on paper = creating a Future
Actually cooking the recipe = .await-ing the Future

You can write 10 recipes, then decide:

Cook them one by one (.await sequentially)
Cook them all at once (join!)
Race them (select!)

Creation is separate from execution!
 */

async fn greet(name: &str) -> String {
    println!("   👋 greet() is EXECUTING right now for '{}'", name);
    format!("Hello, {}!", name)
}

async fn process_record(id: u32) -> String {
    println!("   🔧 Processing record #{} RIGHT NOW", id);
    format!("Record #{} processed", id)
}
#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  Pattern 1: Futures Are LAZY                     ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // ══════════════════════════════════════════════
    // EXPERIMENT 1: Proving laziness
    // ══════════════════════════════════════════════
    println!("━━━ Experiment 1: Creating vs Executing ━━━\n");

    println!("1. About to CALL greet()...");

    let future = greet("Guri");  // ← Call the async function
    println!("2. greet() was called. Did it execute?");
    println!("   Look above — no '👋' message! It did NOT run!\n");

    println!("3. Now calling .await to actually execute it...");
    let result = future.await;  // ← NOW it runs

    println!("4. After .await, result = '{}'\n", result);

    // ══════════════════════════════════════════════
    // EXPERIMENT 3: Dropping a future = nothing happened
    // ══════════════════════════════════════════════
    println!("━━━ Experiment 3: Dropping a Future ━━━\n");

    println!("   Creating a future...");
    let unused_future = greet("Nobody");
    println!("   Now DROPPING it without .await...");
    drop(unused_future);
    println!("   Dropped! No '👋' message — the code NEVER ran!");
    println!("   The greeting was never created. The Future just vanished.\n");

    // ══════════════════════════════════════════════
    // EXPERIMENT 4: Async blocks are also lazy
    // ══════════════════════════════════════════════
    println!("━━━ Experiment 4: Async Blocks Are Also Lazy ━━━\n");

    println!("   Creating an async block...");
    let block_future = async {
        println!("   🎯 Inside the async block!");
        42
    };
    println!("   Block created. No '🎯' message yet!\n");

    println!("   Now .await-ing the block...");
    let value = block_future.await;
    println!("   Block returned: {}\n", value);

    // ══════════════════════════════════════════════
    // KEY TAKEAWAYS
    // ══════════════════════════════════════════════
    println!("━━━ Key Takeaways ━━━\n");
    println!("   1. Calling an async fn creates a Future, does NOT run it");
    println!("   2. .await is what drives the Future to completion");
    println!("   3. Dropping a Future = code inside NEVER executes");
    println!("   4. Async blocks (async {{ }}) are also lazy futures");
    println!("   5. This laziness gives you CONTROL over execution strategy"); 
}