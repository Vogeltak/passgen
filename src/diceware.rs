use std::collections::HashMap;

use rand::{distributions::Uniform, rngs::ThreadRng, Rng};

const WORDLIST: &str = include_str!("../eff_large_wordlist.txt");

/// Method for creating memorable passphrases.
///
/// Uses the user-space CSPRNG ThreadRng under the hood to simulate dice rolls
/// to pick words from the EFF large wordlist.
/// See <https://en.wikipedia.org/wiki/Diceware>.
pub struct Diceware<'a> {
    dist: Uniform<usize>,
    rng: ThreadRng,
    word_map: HashMap<&'a str, &'a str>,
}

impl<'a> Diceware<'a> {
    pub fn new() -> Self {
        // Simulating a six-sided die
        let dist = Uniform::from(1..7);
        let rng = rand::thread_rng();

        let word_map = parse_wordlist(WORDLIST);

        Self {
            dist,
            rng,
            word_map,
        }
    }
}

impl<'a> Iterator for Diceware<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let dice: String = (&mut self.rng)
            .sample_iter(self.dist)
            .take(5)
            .map(|i| i.to_string())
            .collect();

        Some(self.word_map[dice.as_str()].to_string())
    }
}

fn parse_wordlist(wordlist: &str) -> HashMap<&str, &str> {
    let mut res = HashMap::new();

    wordlist
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .for_each(|(dice, word)| {
            res.insert(dice, word);
        });

    res
}
