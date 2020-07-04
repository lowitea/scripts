use clap::Clap;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clap)]
#[clap()]
struct Opts {
    string: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("{}", opts.string.graphemes(true).count())
}
