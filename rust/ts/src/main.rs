use std::process;

use chrono::prelude::DateTime;
use chrono::{Local, TimeZone};
use clap::Clap;

// Constants
const MS_IN_SEC: i32 = 1000;
const MINUTE: i32 = 60;
const HOUR: i32 = MINUTE * 60;
const DAY: i32 = HOUR * 24;
const WEEK: i32 = DAY * 7;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short = "s", long = "seconds")]
    seconds: Option<String>,
    #[clap(short = "m", long = "milliseconds")]
    milliseconds: Option<String>,
    #[clap(short = "d", long = "date", help = "Format: 2014-11-28T12:00:09.2342Z")]
    date: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let is_no_args = opts.seconds.is_none() && opts.milliseconds.is_none() && opts.date.is_none();

    if is_no_args {
        let now = Local::now();
        println!("Constants");
        println!("start epoch:  Thursday, January 1, 1970 12:00:00 AM");
        println!("1 sec:        {} milliseconds", MS_IN_SEC);
        println!("1 minute:     {} seconds", MINUTE);
        println!("1 hour:       {} seconds", HOUR);
        println!("1 day:        {} seconds", DAY);
        println!("1 week:       {} seconds", WEEK);
        println!("30 days:      {} seconds", DAY * 30);
        println!("365 days:     {} seconds", DAY * 365);
        println!("");
        println!("Now");
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
        let dt: DateTime<Local> = date.parse().expect("Bad datetime");
        println!("seconds:      {}", dt.timestamp());
        println!("milliseconds: {}", dt.timestamp_millis());
    });
}
