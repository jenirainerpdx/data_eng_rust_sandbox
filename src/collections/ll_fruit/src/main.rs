use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::{rng, Rng};
use std::collections::LinkedList;
use rand::prelude::IndexedRandom;

// There is no built in shuffle for LinkedLists. It wouldn't make sense.
// LinkedList is not suited for shuffling due to slow access, pointer overhead,
// and scattered memory—swapping elements is too expensive.
// This is just an exercise.

// Insert a value at a specific index in a LinkedList without unstable cursors.
// Strategy: pop elements from the front up to `index`, push the new value,
// then push the rest back while preserving order. If index > len, append.
fn insert_at<T>(list: &mut LinkedList<T>, index: usize, value: T) {
    let len = list.len();
    if index == 0 {
        list.push_front(value);
        return;
    }
    if index >= len {
        list.push_back(value);
        return;
    }

    let mut tmp: LinkedList<T> = LinkedList::new();
    // Move first `index` elements into tmp
    for _ in 0..index {
        if let Some(v) = list.pop_front() {
            tmp.push_back(v);
        }
    }
    // Insert the new value
    tmp.push_back(value);
    // Move the rest of the original list
    while let Some(v) = list.pop_front() {
        tmp.push_back(v);
    }
    // Move everything back to original list
    while let Some(v) = tmp.pop_front() {
        list.push_back(v);
    }
}

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");
    
    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Insert at a specific index (e.g., at index 2)
    insert_at(&mut fruit, 2, "Kiwi");
    insert_at(&mut fruit, 4, "Pear");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // choose a random fruit
    let mut more_fruit: Vec<&str> = fruit.into_iter().collect();
    let pic_one = more_fruit.choose(&mut rng).unwrap();
    println!("Picked a fruit: {}", pic_one);

    let third_fruit = more_fruit.get(3).unwrap();
    let removed = more_fruit.remove(3);
    println!("Removed: {}", removed);
    println!("{:?}", more_fruit);
}