use clap::Parser;

mod cli;
mod password;

fn main() {
    let args = cli::Args::parse();

    let (entropy, pwd) = password::generate(&args);

    println!("{pwd}");
    eprintln!("Entropy: {entropy} bits");
}
