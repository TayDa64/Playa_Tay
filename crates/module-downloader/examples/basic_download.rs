//! Example demonstrating basic usage of the module-downloader crate.
//!
//! This example shows how to:
//! - Configure a downloader
//! - Download a file with progress tracking
//! - Verify integrity with checksums
//!
//! Usage:
//!   cargo run --example basic_download -- <url> <destination>
//!
//! Example:
//!   cargo run --example basic_download -- https://httpbin.org/bytes/1024 /tmp/test.bin

use module_downloader::{DownloadConfig, Downloader, ProgressInfo};
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <url> <destination>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} https://httpbin.org/bytes/1024 /tmp/test.bin", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let dest = Path::new(&args[2]);

    println!("Downloading from: {}", url);
    println!("Destination: {:?}", dest);
    println!();

    // Configure the downloader
    let mut config = DownloadConfig::default();
    config.max_retries = 3;
    config.rate_limit_bps = Some(1_000_000); // 1 MB/s limit (optional)
    
    // You can also specify an expected checksum for verification
    // config.expected_checksum = Some("your_expected_sha256_here".to_string());

    // Create the downloader
    let downloader = Downloader::new(config)?;

    // Download with progress callback
    println!("Starting download...");
    let result = downloader
        .download(
            url,
            dest,
            Some(|progress: ProgressInfo| {
                print!(
                    "\rProgress: {:.1}% ({} / {} bytes)",
                    progress.percentage(),
                    progress.downloaded_bytes,
                    progress.total_bytes
                );
            }),
        )
        .await?;

    println!("\n");
    println!("✓ Download completed successfully!");
    println!("  Path: {:?}", result.path);
    println!("  Size: {} bytes", result.bytes_downloaded);
    println!("  SHA-256: {}", result.checksum);
    println!("  Retries: {}", result.retries_used);
    println!("  Resumed: {}", result.was_resumed);

    Ok(())
}
