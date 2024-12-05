use imap::protocol::{
    connection::Client,
    message::Message,
    error::ProtocolError,
};
use std::time::{Duration, Instant};
use tokio;
use futures::future::join_all;
use tokio::sync::Semaphore;
use std::sync::{Arc, Mutex};

const MESSAGE_COUNT: usize = 10_000;
const PAYLOAD_SIZE: usize = 1024;  // 1KB
const BATCH_SIZE: usize = 100;
const CONCURRENT_CONNECTIONS: usize = 8;
const TIMEOUT_DURATION: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting benchmark...");
    
    let semaphore = Arc::new(Semaphore::new(CONCURRENT_CONNECTIONS));
    let payload = Arc::new("x".repeat(PAYLOAD_SIZE));
    
    let start = Instant::now();
    let latencies = Arc::new(Mutex::new(Vec::with_capacity(MESSAGE_COUNT)));
    
    let mut batch_handles = Vec::new();
    for batch_start in (0..MESSAGE_COUNT).step_by(BATCH_SIZE) {
        let permit = semaphore.clone().acquire_owned().await?;
        let payload = payload.clone();
        let latencies = latencies.clone();
        
        let handle = tokio::spawn(async move {
            let mut connection = Client::connect("127.0.0.1:8080").await?;
            let batch_end = (batch_start + BATCH_SIZE).min(MESSAGE_COUNT);
            let mut batch_messages = Vec::with_capacity(BATCH_SIZE);
            
            for i in batch_start..batch_end {
                batch_messages.push(Message::new(i as u64, (*payload).clone())
                    .with_header("benchmark", "true"));
            }
            
            let batch_start_time = Instant::now();
            for msg in batch_messages {
                if let Ok(Ok(_)) = tokio::time::timeout(
                    TIMEOUT_DURATION,
                    connection.send(&msg)
                ).await {
                    let mut latencies = latencies.lock().unwrap();
                    latencies.push(batch_start_time.elapsed());
                }
            }
            connection.flush().await?;
            drop(permit);
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        });
        
        batch_handles.push(handle);
    }

    join_all(batch_handles).await;
    
    let latencies_guard = latencies.lock().unwrap();
    if latencies_guard.is_empty() {
        println!("No valid measurements collected!");
        return Ok(());
    }

    let total_duration = start.elapsed();
    let actual_count = latencies_guard.len();
    let throughput = actual_count as f64 / total_duration.as_secs_f64();
    
    let bytes_per_message = PAYLOAD_SIZE + 100;
    let mb_per_sec = (throughput * bytes_per_message as f64) / (1024.0 * 1024.0);

    let mut latencies_vec = latencies_guard.clone();
    latencies_vec.sort();
    
    let avg_latency = latencies_vec.iter().sum::<Duration>() / actual_count as u32;
    let p50_latency = latencies_vec[actual_count / 2];
    let p95_latency = latencies_vec[(actual_count as f64 * 0.95) as usize];
    let p99_latency = latencies_vec[(actual_count as f64 * 0.99) as usize];

    println!("\nBenchmark Results:");
    println!("Messages processed: {}/{}", actual_count, MESSAGE_COUNT);
    println!("Total time: {:.2} seconds", total_duration.as_secs_f64());
    println!("Throughput: {:.2} messages/sec", throughput);
    println!("Throughput: {:.2} MB/sec", mb_per_sec);
    println!("Average latency: {:.2} µs", avg_latency.as_micros() as f64);
    println!("P50 latency: {} µs", p50_latency.as_micros());
    println!("P95 latency: {} µs", p95_latency.as_micros());
    println!("P99 latency: {} µs", p99_latency.as_micros());

    Ok(())
}