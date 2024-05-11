use clap::Parser;

#[derive(Parser)]
#[command(version, about="Rust echo", long_about = None)]
struct Cli {
    /// Text to output
    #[arg(required(true))]
    text: Vec<String>,

    /// Don't print the trailing newline character
    #[arg(short)]
    n: bool,
}
fn main() {
    let cli = Cli::parse();

    print!(
        "{}{}",
        cli.text.join(" "),
        if cli.n == false { "\n" } else { "" }
    );
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
