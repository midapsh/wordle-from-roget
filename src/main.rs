const GAMES: &str = include_str!("../data/wordle.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = wordle_from_roget::algorithms::Naive::new();
        wordle_from_roget::play(answer, guesser);
    }
    println!("Hello, world!");
}
