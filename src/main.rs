// filepath: [main.rs](http://_vscodecontentref_/10)
mod icmp;
mod config;
mod ping;
mod report;

use clap::{Arg, ArgAction, Command};
use ping::send_multiple_pings;

fn main() {
    // ----------------------- Argument Parsing -----------------------
    let matches = Command::new("rs_ping")
        .version("0.1.0")
        .author("Your Name <rszylkowski1@gmail.com>")
        .about("A Rust-based ICMP packet sender. Requires sudo to run.")
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Number of packets to send")
                .default_value(config::DEFAULT_PACKET_COUNT_STR),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .value_name("INTERVAL")
                .help("Interval between packets in milliseconds")
                .default_value(config::DEFAULT_INTERVAL_MS_STR),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .value_name("DELAY")
                .help("Delay before starting to send packets in milliseconds")
                .default_value(config::DEFAULT_DELAY_MS_STR),
        )
        .arg(
            Arg::new("debug")
                .short('D')
                .long("debug")
                .help("Enable debug output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("report")
                .short('r')
                .long("report")
                .help("Generate a CSV report")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no_wait")
                .short('n')
                .long("no-wait")
                .help("Do not wait for a response")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("faf")
                .short('f')
                .long("faf")
                .help("Enable multithreading")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .value_name("THREADS")
                .help("Number of threads to use (requires --faf)")
                .default_value("1"),
        )
        .arg(
            Arg::new("target")
                .help("Target IP address")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Parse arguments
    let count: u32 = matches
        .get_one::<String>("count")
        .unwrap()
        .parse()
        .expect("Invalid count value");
    let interval: u64 = matches
        .get_one::<String>("interval")
        .unwrap()
        .parse()
        .expect("Invalid interval value");
    let delay: u64 = matches
        .get_one::<String>("delay")
        .unwrap()
        .parse()
        .expect("Invalid delay value");
    let target = matches.get_one::<String>("target").unwrap();
    let no_wait = matches.get_flag("no_wait");
    let debug = matches.get_flag("debug");
    let report = matches.get_flag("report");
    let faf = matches.get_flag("faf");
    let threads: u32 = matches.get_one::<String>("threads").unwrap().parse().expect("Invalid threads value");

    // Call the ping functionality
    send_multiple_pings(count, interval, delay, target, no_wait, debug, report, faf, threads);
}