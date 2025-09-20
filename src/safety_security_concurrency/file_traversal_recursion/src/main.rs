use clap::Parser;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(required = true)]
    directory: String,
}

fn main() {
    let directory = Cli::parse().directory;

    let file_count = Arc::new(AtomicU64::new(0));
    let total_size = Arc::new(AtomicU64::new(0));

    let start_time = std::time::Instant::now();
    // Simple recursive approach
    traverse_directory(Path::new(&directory), &file_count, &total_size);

    println!("Files: {}", file_count.load(Ordering::Relaxed));
    println!("Total size: {} bytes", total_size.load(Ordering::Relaxed));
    println!("Elapsed: {:?}", start_time.elapsed());
}

fn traverse_directory(dir: &Path, file_count: &Arc<AtomicU64>, total_size: &Arc<AtomicU64>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = entry.metadata() {
                    file_count.fetch_add(1, Ordering::Relaxed);
                    total_size.fetch_add(metadata.len(), Ordering::Relaxed);
                }
            } else if path.is_dir() {
                traverse_directory(&path, file_count, total_size); // Recursion!
            }
        }
    }
}
