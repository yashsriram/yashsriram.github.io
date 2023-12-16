use rand::{distributions::WeightedIndex, prelude::*};

fn main() {
    let mut rng = thread_rng();
    let weathers = ["🟨", "🟩", "🟦"];
    let T = [
        WeightedIndex::new([0.8, 0.2, 0.0]).unwrap(),
        WeightedIndex::new([0.4, 0.4, 0.2]).unwrap(),
        WeightedIndex::new([0.2, 0.6, 0.2]).unwrap(),
    ];
    for _ in 0..25 {
        let mut seed = [0, 1, 2];
        seed.shuffle(&mut rng);
        let mut today = seed[0];
        for _ in 0..50 {
            print!("{}", weathers[today]);
            let tomorrow = T[today].sample(&mut rng);
            today = tomorrow;
        }
        println!("{}", weathers[today]);
    }
}
