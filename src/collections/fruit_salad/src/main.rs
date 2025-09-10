use clap::Parser;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::{Rng, rng};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Fruits to include in the salad. Examples:
    ///   --fruits apple orange mango
    ///   --fruits apple,orange,mango
    /// You can pass between 1 and 5 items.
    #[arg(
        long,
        short = 'f',
        value_delimiter = ',',
        num_args = 1..=5,
        required = true
    )]
    fruits_input: Vec<String>,
}

enum EsotericFruits {
    Pomegranate,
    Guava,
    Pawpaw,
    Durian,
    Mangosteen,
    Cupuacu,
}

impl EsotericFruits {
    fn from_ordinal(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::Pomegranate),
            1 => Some(Self::Guava),
            2 => Some(Self::Pawpaw),
            3 => Some(Self::Durian),
            4 => Some(Self::Mangosteen),
            5 => Some(Self::Cupuacu),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Pomegranate => "Pomegranate",
            Self::Guava => "Guava",
            Self::Pawpaw => "Pawpaw",
            Self::Durian => "Durian",
            Self::Mangosteen => "Mangosteen",
            Self::Cupuacu => "Cupuacu",
        }
    }
}

fn main() {
    let mut fruits = Cli::parse().fruits_input;

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    fruits.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Add 5 random from esoteric fruits
    for _ in 0..5 {
        let random_num = rng.random_range(0..=5);
        let esoteric_fruit = EsotericFruits::from_ordinal(random_num).unwrap();
        fruits.push(esoteric_fruit.as_str().to_string());
    }
    println!("Fruit Salad: {:?}", fruits);
    println!("Picked a fruit: {}", fruits.choose(&mut rng).unwrap())
}
