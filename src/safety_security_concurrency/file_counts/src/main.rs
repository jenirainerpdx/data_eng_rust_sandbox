use clap::Parser;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(required = true)]
    directory: String,
}

// Visits all files and dirs under `path` using Rayon parallel recursion.
// Returns (file_count, total_size)
fn visit_dir(path: &Path, file_count: &Arc<AtomicU64>, total_size: &Arc<AtomicU64>) {
    // List entries, collect as Vec (avoid borrowing issues)
    let entries = match fs::read_dir(path) {
        Ok(e) => e.collect::<Result<Vec<_>, _>>().unwrap_or_default(),
        Err(_) => return,
    };

    // Do files (sequentially, but can be par_iter if dirs are rare)
    for entry in &entries {
        let path = entry.path();
        if path.is_file()
            && let Ok(metadata) = entry.metadata()
        {
            file_count.fetch_add(1, Ordering::Relaxed);
            total_size.fetch_add(metadata.len(), Ordering::Relaxed);
        }
    }
    // Recurse into subdirs in parallel (using par_iter)
    entries
        .par_iter()
        .filter(|entry| entry.path().is_dir())
        .for_each(|entry| {
            visit_dir(&entry.path(), file_count, total_size);
        });
}

fn main() {
    let args = Cli::parse();
    let root = PathBuf::from(args.directory);

    let file_count = Arc::new(AtomicU64::new(0));
    let total_size = Arc::new(AtomicU64::new(0));

    let start_time = std::time::Instant::now();
    visit_dir(&root, &file_count, &total_size);

    println!("Files: {}", file_count.load(Ordering::Relaxed));
    println!("Total size: {} bytes", total_size.load(Ordering::Relaxed));
    println!("Elapsed: {:?}", start_time.elapsed());
}
