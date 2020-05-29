//! ToDo: поправить подсчёт количества русских букв

use clap::Clap;

#[derive(Clap)]
#[clap()]
struct Opts {
    string: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("{}", opts.string.len())
}
