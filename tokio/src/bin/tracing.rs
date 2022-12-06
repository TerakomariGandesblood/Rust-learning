use tracing::{info, instrument, subscriber};

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();

    subscriber::set_global_default(subscriber)?;

    let _ = trace_me(1, 2).await;

    Ok(())
}

#[instrument]
async fn trace_me(a: u32, b: u32) -> u32 {
    info!("a info");
    a + b
}
