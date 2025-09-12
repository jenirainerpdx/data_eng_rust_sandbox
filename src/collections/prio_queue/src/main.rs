use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
struct Job {
    priority: usize,
    name: String,
}

// Implement ordering for BinaryHeap (max-heap by priority)
impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut pq = BinaryHeap::new();
    pq.push(Job {
        priority: 2,
        name: "low".to_string(),
    });
    pq.push(Job {
        priority: 5,
        name: "high".to_string(),
    });
    pq.push(Job {
        priority: 3,
        name: "medium".to_string(),
    });
    pq.push(Job {
        priority: 3,
        name: "other medium".to_string(),
    });

    while let Some(job) = pq.pop() {
        println!("Priority: {}, Name: {}", job.priority, job.name);
    }
}
