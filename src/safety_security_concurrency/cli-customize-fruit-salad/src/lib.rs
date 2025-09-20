use rand::rng;
use rand::seq::SliceRandom;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = rng();
    fruits.shuffle(&mut rng);

    fruits
}
