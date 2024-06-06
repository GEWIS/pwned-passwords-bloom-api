use rustis::{client::Client, commands::{FlushingMode, ServerCommands, BloomCommands, BfReserveOptions}, Result};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    // The number of hashes to be in the bloom filter and the probability for false positives.
    let n = 936491757;
    let p = 0.00000001;
    let batch_size = 1_000_000;

    // Connect to local Redis (Stack) server with support for RedisBloom.
    let client = Client::connect("redis://127.0.0.1:6379").await?;
    client.flushdb(FlushingMode::Sync).await?;

    // Reserve the RedisBloom filter with the appropriate requirements.
    client.bf_reserve("pwned-bloom", p, n, BfReserveOptions::default().nonscaling()).await?;

    let start = Instant::now();

    let mut count = 0;
    let mut batch = Vec::new();

    if let Ok(lines) = read_lines("./pwnedpasswords.sha1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let hash = ip.split(':').next().unwrap_or(&ip);
                batch.push(hash.to_string());
                count += 1;

                if count % batch_size == 0 {
                    client.bf_madd("pwned-bloom", batch).await?;
                    batch = Vec::new();

                    let elapsed = start.elapsed();
                    println!("Count: {}, Time elapsed: {:?}", count, elapsed);
                }
            }
        }
    }

    // Add any remaining items in the batch.
    if !batch.is_empty() {
        client.bf_madd("pwned-bloom", batch).await?;
        let elapsed = start.elapsed();
        println!("Count: {}, Time elapsed: {:?}", count, elapsed);
    }

    let end = Instant::now();
    println!("Total execution time: {}m", (end - start).as_secs() / 60);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}