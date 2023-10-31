use clap::{Parser, ValueEnum};

/// Configurable password generator
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The type of password to generate.
    #[arg(value_enum, default_value_t = PasswordType::Random)]
    pub password_type: PasswordType,

    /// Desired length of the password.
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    /// Exclude numbers from the password.
    #[arg(long)]
    pub no_numbers: bool,

    /// Exclude special symbols from the password.
    #[arg(long)]
    pub no_symbols: bool,

    /// Exclude capital letters from the password.
    #[arg(long)]
    pub no_capitals: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PasswordType {
    /// Normal password
    Random,
    /// Password consisting only of numerals
    Pin,
    /// Password consisting only of memorable words
    Memorable,
}
