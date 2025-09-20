use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn increment_using_mutex(data_vec: Vec<i32>) {
    let data = Arc::new(Mutex::new(data_vec));
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut locked = data.lock().unwrap();
            locked[i] += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", data.lock().unwrap());
}

fn increment_moving_values(data_vec: Vec<i32>) {
    let mut handles = vec![];
    for value in data_vec {
        handles.push(thread::spawn(move || {
            value + 1
        }));
    }
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap()).collect();
    println!("{:?}", results);
}

fn main() {
    let data_vec = vec![1, 2, 3];
    increment_using_mutex(data_vec);

    increment_moving_values(vec![1, 2, 3]);

    increment_using_rwlock(vec![1,2,3]);


   /* for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        thread::spawn(move || {
            data[i] += 1;
        });
    }*/

    // No data race can occur, this will not compile.
}

// Of course this doesn't make any sense. We are only writing, so we would not do this.
// But, for learning syntax, I'm doing it anyway.
fn increment_using_rwlock(data_vec: Vec<i32>) {
    let data = Arc::new(RwLock::new(data_vec));
    let mut handles = vec![];
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut locked = data.write().unwrap();
            locked[i] += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let result = data.read().unwrap();
    println!("{:?}", *result);
}
