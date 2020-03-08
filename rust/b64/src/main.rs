use clap::Clap;
use std::str;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short = "d", long = "decode")]
    decode: bool,
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.decode {
        true => {
            let buf = base64::decode(&opts.input)
                .expect("Decoding fail");
            let string = str::from_utf8(&buf)
                .expect("Decoding fail");
            println!("{}", string)
        }
        false => {println!("{}", base64::encode(&opts.input))}
    }
}
