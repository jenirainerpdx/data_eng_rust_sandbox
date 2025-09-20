/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers always acquire the lower-numbered fork first. This breaks
* circular waiting regardless of the number of philosophers or forks.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/

use std::collections::VecDeque;
use std::env;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Fork {
    id: u32,
    state: Mutex<ForkState>,
    condvar: Condvar,
}

struct ForkState {
    available: bool,
    wait_queue: VecDeque<u32>, // philosopher IDs waiting
}

impl Fork {
    fn new(id: u32) -> Self {
        Fork {
            id,
            state: Mutex::new(ForkState {
                available: true,
                wait_queue: VecDeque::new(),
            }),
            condvar: Condvar::new(),
        }
    }
    
    fn acquire(&self, philosopher_id: u32) {
        let mut state = self.state.lock().unwrap();
        state.wait_queue.push_back(philosopher_id);
        
        while !state.available || state.wait_queue.front() != Some(&philosopher_id) {
            state = self.condvar.wait(state).unwrap();
        }
        
        state.available = false;
        state.wait_queue.pop_front();
    }
    
    fn release(&self) {
        let mut state = self.state.lock().unwrap();
        state.available = true;
        self.condvar.notify_all();
    }
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(id: u32, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
        Philosopher {
            id,
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

        first_fork.acquire(self.id);
        println!("{} picked up fork {}.", self.name, first_fork.id);
        second_fork.acquire(self.id);
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
    
    println!("Dining Philosophers Problem: {} Philosophers with {} Forks", num_philosophers, num_forks);

    let forks = (0..num_forks)
        .map(|id| Arc::new(Fork::new(id as u32)))
        .collect::<Vec<_>>();

    let philosophers = (0..num_philosophers)
        .map(|id| {
            let left_fork = id % num_forks;
            let right_fork = (id + 1) % num_forks;
            Philosopher::new(
                id as u32,
                &format!("Philosopher {}", id),
                Arc::clone(&forks[left_fork]),
                Arc::clone(&forks[right_fork]),
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
