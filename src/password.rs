use rand::distributions::Uniform;
use rand::Rng;

use crate::cli::Args;

const NUMERALS: &[u8] = b"0123456789";
const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const ALPHABET_CAPITALIZED: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SYMBOLS: &[u8] = b" !\"#$%'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn generate(args: &Args) -> (f64, String) {
    use crate::cli::PasswordType::*;
    let character_set = match args.password_type {
        Random => {
            let mut character_set = ALPHABET.to_vec();
            if !args.no_numbers {
                character_set.extend_from_slice(NUMERALS);
            }
            if !args.no_symbols {
                character_set.extend_from_slice(SYMBOLS);
            }
            if !args.no_capitals {
                character_set.extend_from_slice(ALPHABET_CAPITALIZED);
            }

            character_set
        }
        Pin => NUMERALS.to_vec(),
        Memorable => {
            todo!()
        }
    };

    let dist = Uniform::from(0..character_set.len());
    // ThreadRng is a user-space CSPRNG (i.e. implements CryptoRng trait)
    let mut rng = rand::thread_rng();

    // Strength as measured by the information entropy is the base-2 logarithm
    // of the number of possible passwords. See:
    // https://en.wikipedia.org/wiki/Password_strength#Random_passwords
    let entropy = args.length as f64 * (character_set.len() as f64).log2() / 2_f64.log2();

    (
        entropy,
        (&mut rng)
            .sample_iter(dist)
            .take(args.length)
            .map(|i| char::from(character_set[i]))
            .collect::<String>(),
    )
}
