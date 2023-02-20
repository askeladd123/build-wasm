use std::iter::Iterator;

mod cli;
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    cli::run(args.iter().map(|v| v.as_str()));
}
