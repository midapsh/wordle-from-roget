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

impl Guess {
    pub fn matches(&self, word: &str) -> bool {
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);

        // First, check greens
        let mut used = [false; 5];
        for (i, ((g, &m), w)) in self
            .word
            .chars()
            .zip(&self.mask)
            .zip(word.chars())
            .enumerate()
        {
            if m == Correctness::Correct {
                if g != w {
                    return false;
                } else {
                    used[i] = true;
                }
            }
        }

        for (i, (w, &m)) in word.chars().zip(&self.mask).enumerate() {
            if m == Correctness::Correct {
                // Must be correct, or we'd have returned in the earlier
                // loop
                continue;
            }

            let mut plausible = true;
            // Find the first unused occurrence of the current character
            // in the previous guess
            if self
                .word
                .chars()
                .zip(&self.mask)
                .enumerate()
                .any(|(j, (g, m))| {
                    if g != w {
                        return false;
                    }
                    if used[j] {
                        // Can't use this to support this character
                        return false;
                    }
                    // We're looking at an `w` in `word`, and have found an `w`
                    // in the previous guess.
                    // The collor of that previous `w` will tell us whether this
                    // `w` _might_ be okay.
                    match m {
                        Correctness::Correct => unreachable!(
                            "All correct guesses should have result in return or be used"
                        ),
                        Correctness::Misplaced if j == i => {
                            // `w` was yellow in this same position last time around,
                            // which means that `word` _cannot_ be the answer.
                            plausible = false;
                            return false;
                        }
                        Correctness::Misplaced => {
                            used[j] = true;
                            return true;
                        }
                        Correctness::Wrong => {
                            // TODO: early return
                            plausible = false;
                            return false;
                        }
                    }
                })
                && plausible
            {
                // The character `w` was yellow in the previous guess
            } else if !plausible {
                return false;
            } else {
                // We have no information about character `w`, so word might
                // still match.
            }
        }
        true
    }
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

    macro_rules! mask {
        (C) => {crate::Correctness::Correct};
        (M) => {crate::Correctness::Misplaced};
        (W) => {crate::Correctness::Wrong};
        ($($c:tt)+) => {[
            $(mask!($c)),+
        ]}
    }
    mod guess_matcher {
        use crate::Guess;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                    word: $prev.to_string(),
                    mask: mask![$($mask )+]
                }
                .matches($next));
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                    word: $prev.to_string(),
                    mask: mask![$($mask )+]
                }
                .matches($next));
            }
        }

        #[test]
        fn test_basic_match() {
            check!("abcde" + [C C C C C] allows "abcde");
            check!("abcdf" + [C C C C C] disallows "abcde");
            check!("abcde" + [W W W W W] allows "fghij");
            check!("abcde" + [M M M M M] allows "eabcd");
            check!("baaaa" + [W C M W W] allows "aaccc");
            check!("baaaa" + [W C M W W] disallows "caacc");
        }

        #[test]
        fn test_from_joe_gjengset_chat() {
            // flocular
            check!("aaabb" + [C M W W W] disallows "accaa");
            // ritoban
            check!("abcde" + [W W W W W] disallows "bcdea");
        }
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
