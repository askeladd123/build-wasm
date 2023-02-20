mod cli;
pub fn run<'a>(args: impl IntoIterator<Item = &'a str>) {
    cli::run(args);
}
