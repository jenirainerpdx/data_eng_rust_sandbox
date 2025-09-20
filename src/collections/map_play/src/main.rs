use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

fn main() {
    let mut map: HashMap<&Person, u8> = std::collections::HashMap::new();
    let john = Person::new("John".to_string(), 20);
    map.entry(&john).or_insert(0);
    let jane = Person::new("Jane".to_string(), 21);
    map.entry(&jane).and_modify(|jane| *jane += 1).or_insert(5);
    let ben = Person::new("Ben".to_string(), 22);
    map.entry(&ben).or_insert(7);
    println!("{:?}", map);

    map.entry(&jane).and_modify(|jane| *jane += 50);
    println!("{:?}", map.entry(&jane));

}
