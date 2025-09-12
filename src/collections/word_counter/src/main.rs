use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(value_name = "file", short = 'f')]
    filename: String,
}

// enter a filename.
// reads the file and counts the words.
fn main() {
    let filename = Cli::parse().filename;
    println!("{:?}", filename);

    let file = File::open(filename);
    let reader = std::io::BufReader::new(file.unwrap());

    let mut word_count: HashMap<String, i32> = HashMap::new();
    for line in reader.lines() {
        let aline = line.unwrap().to_string();
        let words: Vec<String> = aline.split_whitespace().map(|s| s.to_string()).collect();
        for word in words {
            *word_count.entry(word).or_insert(0) += 1;
        }
        
    }
    println!("{:?}", word_count);
}
