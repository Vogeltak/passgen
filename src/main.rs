//! A small and nimble executable to generate secure passwords.
//!
//! # Examples
//!
//! ```text
//! $ passgen -h
//! Configurable password generator
//!
//! Usage: passgen [OPTIONS]
//!
//! Options:
//!   -t <PASSWORD_TYPE>      The type of password to generate [default: random] [possible values: random, pin, memorable]
//!   -l, --length <LENGTH>   Desired length of the password [default: 16]
//!       --no-numbers        Exclude numbers from the password
//!       --no-symbols        Exclude special symbols from the password
//!       --no-capitals       Exclude capital letters from the password
//!   -h, --help              Print help (see more with '--help')
//!   -V, --version           Print version
//! ```

use anyhow::Result;
use clap::Parser;

mod cli;
mod diceware;
mod password;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    let pwd = password::generate(&args)?;

    println!("{}", pwd.text);
    eprintln!("Entropy: {} bits", pwd.entropy);

    Ok(())
}
