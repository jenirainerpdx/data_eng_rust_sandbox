use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author,version,about)]
struct Cli {
    #[arg(long, value_enum)]
    collection: CollectionType,
    #[arg(long)]
    count: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum CollectionType {
    Vec,
    VecDeque,
    LinkedList,
    HashSet,
    HashMap,
}

fn main() {
    let cli = Cli::parse();

    match cli.collection {
        CollectionType::Vec => {
            let mut c = Vec::with_capacity(cli.count);
            for i in 0..cli.count {
                c.push(i);
            }
            println!("Vec created with {} elements, {:?}", c.len(), c);
        }
        CollectionType::VecDeque => {
            let mut c = std::collections::VecDeque::with_capacity(cli.count);
            for i in 0..cli.count {
                c.push_back(i);
            }
            println!("VecDeque created with {} elements, {:?}", c.len(), c);
        }
        CollectionType::LinkedList => {
            let mut c = std::collections::LinkedList::new();
            for i in 0..cli.count {
                c.push_back(i);
            }
            println!("LinkedList created with {} elements, {:?}", c.len(), c);
        }
        CollectionType::HashSet => {
            let mut c = std::collections::HashSet::new();
            for i in 0..cli.count {
                c.insert(i);
            }
            println!("HashSet created with {} elements, {:?}", c.len(), c);
        }
        CollectionType::HashMap => {
            let mut c = std::collections::HashMap::new();
            for i in 0..cli.count {
                c.insert(i, "no value");
            }
            println!("HashMap created with {} elements, {:?}", c.len(), c);
        }
    }
    println!("{:?}", cli);
    println!("done");
}
