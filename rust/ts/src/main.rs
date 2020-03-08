use std::process;

use chrono::{Local, TimeZone};
use chrono::prelude::DateTime;
use clap::Clap;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short = "s", long = "seconds")]
    seconds: Option<String>,
    #[clap(short = "m", long = "milliseconds")]
    milliseconds: Option<String>,
    #[clap(short = "d",
           long = "date",
           help = "Format: 2014-11-28T12:00:09.2342Z")]
    date: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let is_no_args = opts.seconds.is_none()
        && opts.milliseconds.is_none()
        && opts.date.is_none();

    if is_no_args {
        let now = Local::now();
        println!("seconds:      {}", now.timestamp());
        println!("milliseconds: {}", now.timestamp_millis());
        println!("date:         {}", now);
        process::exit(0);
    };

    opts.seconds.map(|ts| {
        let date: i64 = ts.parse().expect("Bad seconds");
        println! {"{}", Local.timestamp(date, 0)};
    });

    opts.milliseconds.map(|ms| {
        let date: i64 = ms.parse().expect("Bad milliseconds");
        println! {"{}", Local.timestamp_millis(date)};
    });

    opts.date.map(|date| {
        let dt: DateTime<Local> = date.parse()
            .expect("Bad datetime");
        println!("seconds:      {}", dt.timestamp());
        println!("milliseconds: {}", dt.timestamp_millis());
    });
}
