# module-downloader

HTTP downloader with resume capability and integrity verification for the Tauri module system.

## Features

- **Resumable Downloads**: Automatically resumes interrupted downloads using HTTP range requests
- **Integrity Verification**: SHA-256 checksum validation to ensure download integrity
- **Rate Limiting**: Optional bandwidth throttling to limit download speed
- **Retry with Backoff**: Automatic retry with exponential backoff on transient failures
- **Progress Tracking**: Callback-based progress reporting
- **Temp File Management**: Safe handling of temporary files with automatic cleanup

## Usage

```rust
use module_downloader::{Downloader, DownloadConfig, ProgressInfo};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a downloader with custom configuration
    let mut config = DownloadConfig::default();
    config.max_retries = 5;
    config.rate_limit_bps = Some(1_000_000); // 1 MB/s
    config.expected_checksum = Some("abc123...".to_string());
    
    let downloader = Downloader::new(config)?;
    
    // Download with progress callback
    let result = downloader
        .download(
            "https://example.com/large-file.zip",
            Path::new("/tmp/large-file.zip"),
            Some(|progress: ProgressInfo| {
                println!(
                    "Downloaded: {} / {} bytes ({:.1}%)",
                    progress.downloaded_bytes,
                    progress.total_bytes,
                    progress.percentage()
                );
            }),
        )
        .await?;
    
    println!("Download completed: {:?}", result.path);
    println!("Checksum: {}", result.checksum);
    println!("Was resumed: {}", result.was_resumed);
    
    Ok(())
}
```

## Configuration

### `DownloadConfig`

- `max_retries`: Maximum number of retry attempts (default: 3)
- `initial_backoff_ms`: Initial backoff duration in milliseconds (default: 1000)
- `max_backoff_ms`: Maximum backoff duration in milliseconds (default: 32000)
- `rate_limit_bps`: Optional rate limit in bytes per second (default: None)
- `timeout`: Overall timeout for the download operation (default: 1 hour)
- `expected_checksum`: Optional expected SHA-256 checksum as hex string (default: None)

## Resume Behavior

If a download is interrupted, the downloader will:

1. Check if a `.part` file exists at the destination
2. Send an HTTP Range request to resume from the existing file size
3. If the server doesn't support range requests, start over
4. Continue downloading and streaming to the file
5. Verify the checksum of the complete file
6. Atomically rename the `.part` file to the final destination

## Error Handling

The downloader handles various error conditions:

- **Network errors**: Automatic retry with exponential backoff
- **HTTP errors**: Clear error messages with status codes
- **Checksum mismatches**: Validation failure with expected/actual values
- **I/O errors**: File system errors with context

## Performance

- Uses streaming to minimize memory usage
- Buffered writes for efficient I/O
- Optional rate limiting to control bandwidth usage
- LRU token bucket for smooth rate limiting

## Testing

Run the test suite:

```bash
cargo test --package module-downloader
```

## Integration

This crate is designed to be used within the Tauri module update system. It provides
the download primitive for the `update-manager` orchestrator (Phase 2.3 Task 4).

## License

MIT or Apache-2.0
