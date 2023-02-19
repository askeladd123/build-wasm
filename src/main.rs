use std::iter::Iterator;

mod cli;
fn main() {
    cli::run(std::env::args().skip(1));
}
