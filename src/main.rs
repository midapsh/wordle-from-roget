const GAMES: &str = include_str!("../data/wordle.txt");

fn main() {
    let w = wordle_from_roget::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = wordle_from_roget::algorithms::Naive::new();
        w.play(answer, guesser);
    }
    println!("Hello, world!");
}
