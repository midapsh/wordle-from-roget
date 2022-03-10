use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../data/dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("Every line is word + space + frequency")
                    .0
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        // Play six roundas where it invokes guesser each round
        let mut history = Vec::new();
        // Wordle only alllows six guesses.
        // We allow more to avoid choping of the score distribution
        // for stats purposes.
        for i in 1..=32 {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(&(*guess)));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            });
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut hits = [Correctness::Wrong; 5];
        let mut used = [false; 5];

        // Mark things green (correct)
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                hits[i] = Correctness::Correct;
                used[i] = true;
            }
        }

        // Mark things yellow (correct)
        for g in guess.chars() {
            for (i, a) in answer.chars().enumerate() {
                if a == g && !used[i] {
                    used[i] = true;
                    hits[i] = Correctness::Misplaced;
                }
            }
        }

        hits
    }
}

pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
mod tests {
    macro_rules! guesser {
        (|$history:ident| $impl:block) => {{
            struct G;
            impl $crate::Guesser for G {
                fn guess(&mut self, $history: &[$crate::Guess]) -> String {
                    $impl
                }
            }
            G
        }};
    }
    mod game {
        use crate::Wordle;

        #[test]
        fn test_first_try_match() {
            // Genius match
            let w = Wordle::new();
            let guesser_ = guesser!(|history| { "moved".to_string() });
            assert_eq!(w.play("moved", guesser_), Some(1));
        }

        #[test]
        fn test_second_try_match() {
            // Magnificent match
            let w = Wordle::new();
            let guesser_ = guesser!(|history| {
                if history.len() == 1 {
                    "right".to_string()
                } else {
                    "wrong".to_string()
                }
            });
            assert_eq!(w.play("right", guesser_), Some(2));
        }

        #[test]
        fn test_third_try_match() {
            // Impressive match
            let w = Wordle::new();
            let guesser_ = guesser!(|history| {
                if history.len() == 2 {
                    "right".to_string()
                } else {
                    "wrong".to_string()
                }
            });
            assert_eq!(w.play("right", guesser_), Some(3));
        }

        #[test]
        fn test_fourth_try_match() {
            // Splendid match
            let w = Wordle::new();
            let guesser_ = guesser!(|history| {
                if history.len() == 3 {
                    "right".to_string()
                } else {
                    "wrong".to_string()
                }
            });
            assert_eq!(w.play("right", guesser_), Some(4));
        }

        #[test]
        fn test_fifth_try_match() {
            // Great match
            let w = Wordle::new();
            let guesser_ = guesser!(|history| {
                if history.len() == 4 {
                    "right".to_string()
                } else {
                    "wrong".to_string()
                }
            });
            assert_eq!(w.play("right", guesser_), Some(5));
        }

        #[test]
        fn test_sixth_try_match() {
            // Phew match
            let w = Wordle::new();
            let guesser_ = guesser!(|history| {
                if history.len() == 5 {
                    "right".to_string()
                } else {
                    "wrong".to_string()
                }
            });
            assert_eq!(w.play("right", guesser_), Some(6));
        }

        #[test]
        fn test_no_match_found() {
            // Oops
            let w = Wordle::new();
            let guesser_ = guesser!(|history| { "wrong".to_string() });
            assert_eq!(w.play("right", guesser_), None);
        }
    }
    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn test_if_all_correct() {
            let (answer, guess) = ("abcde", "abcde");
            assert_eq!(Correctness::compute(answer, guess), mask![C C C C C])
        }
        #[test]
        fn test_if_it_finds_all_misplaced_chars() {
            let (answer, guess) = ("abcde", "bcdea");
            assert_eq!(Correctness::compute(answer, guess), mask![M M M M M])
        }
        #[test]
        fn test_if_it_finds_all_wrongs_chars() {
            let (answer, guess) = ("aaaaa", "bbbbb");
            assert_eq!(Correctness::compute(answer, guess), mask![W W W W W])
        }
        #[test]
        fn test_if_finds_only_one_correct() {
            let (answer, guess) = ("abcde", "edcba");
            assert_eq!(Correctness::compute(answer, guess), mask![M M C M M])
        }
        #[test]
        fn test_if_it_finds_misplaced_chars_with_right_and_wrong_chars() {
            let (answer, guess) = ("aabbc", "ababw");
            assert_eq!(Correctness::compute(answer, guess), mask![C M M C W])
        }
        #[test]
        #[should_panic]
        fn test_if_answer_is_less_than_5() {
            let (answer, guess) = ("abcd", "abcde");
            Correctness::compute(answer, guess);
        }
        #[test]
        #[should_panic]
        fn test_if_answer_is_more_than_5() {
            let (answer, guess) = ("abcdef", "abcde");
            Correctness::compute(answer, guess);
        }
        #[test]
        #[should_panic]
        fn test_if_guess_is_less_than_5() {
            let (answer, guess) = ("abcde", "abcd");
            Correctness::compute(answer, guess);
        }
        #[test]
        #[should_panic]
        fn test_if_guess_is_more_than_5() {
            let (answer, guess) = ("abcde", "abcdef");
            Correctness::compute(answer, guess);
        }
    }
}
