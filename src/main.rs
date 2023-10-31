use anyhow::Result;
use clap::Parser;

mod cli;
mod password;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    let pwd = password::generate(&args)?;

    println!("{}", pwd.text);
    eprintln!("Entropy: {} bits", pwd.entropy);

    Ok(())
}
