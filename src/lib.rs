mod cli;
pub fn run(args: impl IntoIterator<Item=String>) {
    cli::run(args);
}
