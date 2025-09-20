use std::env;
use std::thread;
use std::time::{Duration, Instant};
use crossbeam::channel::{unbounded, Sender};
use crossbeam::select;

#[derive(Clone)]
struct Fork {
    id: u32,
    acquire_tx: Sender<Sender<()>>,
    release_tx: Sender<()>,
}

impl Fork {
    fn new(id: u32) -> Self {
        let (acquire_tx, acquire_rx) = unbounded::<Sender<()>>();
        let (release_tx, release_rx) = unbounded::<()>();

        // Spawn waiter thread for this fork
        thread::spawn(move || {
            let mut available = true;
            let mut wait_queue = Vec::new();

            loop {
                // Process waiting queue first
                if available && !wait_queue.is_empty() {
                    let response_tx: Sender<()> = wait_queue.remove(0);
                    response_tx.send(()).unwrap();
                    available = false;
                    continue;
                }
                select! {
                    recv(acquire_rx) -> response_tx => {
                        if let Ok(response_tx) = response_tx {
                            if available {
                                response_tx.send(()).unwrap();
                                available = false;
                            } else {
                                wait_queue.push(response_tx);
                            }
                        }
                    }
                    recv(release_rx) -> _ => {
                        available = true;
                    }
                }
            }
        });
            
        Fork {
            id,
            acquire_tx,
            release_tx,
        }
    }

    fn acquire(&self) {
        let (response_tx, response_rx) = unbounded();
        self.acquire_tx.send(response_tx).unwrap();
        response_rx.recv().unwrap(); // Block until fork granted
    }

    fn release(&self) {
        self.release_tx.send(()).unwrap();
    }
}

struct Philosopher {
    name: String,
    left_fork: Fork,
    right_fork: Fork,
}

impl Philosopher {
    fn new(name: &str, left_fork: Fork, right_fork: Fork) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    fn eat(&self) {
        let (first_fork, second_fork) = if self.left_fork.id < self.right_fork.id {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        first_fork.acquire();
        println!("{} picked up fork {}.", self.name, first_fork.id);
        second_fork.acquire();
        println!("{} picked up fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        first_fork.release();
        println!("{} put down fork {}.", self.name, first_fork.id);
        second_fork.release();
        println!("{} put down fork {}.", self.name, second_fork.id);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <num_philosophers> <num_forks>", args[0]);
        std::process::exit(1);
    }

    let num_philosophers: usize = args[1].parse().expect("Invalid number of philosophers");
    let num_forks: usize = args[2].parse().expect("Invalid number of forks");

    println!(
        "Dining Philosophers Problem: {} Philosophers with {} Forks",
        num_philosophers, num_forks
    );

    let forks = (0..num_forks)
        .map(|id| Fork::new(id as u32))
        .collect::<Vec<_>>();

    let philosophers = (0..num_philosophers)
        .map(|id| {
            let left_fork_id = id % num_forks;
            let right_fork_id = (id + 1) % num_forks;

            Philosopher::new(
                &format!("Philosopher {}", id),
                forks[left_fork_id].clone(),
                forks[right_fork_id].clone(),
            )
        })
        .collect::<Vec<_>>();

    let start = Instant::now();

    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                philosopher.eat();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}
