// filepath: [main.rs](http://_vscodecontentref_/0)
//! Entry point for the rs_ping application.

mod icmp;
mod config;

use clap::{Arg, ArgAction, Command};
use icmp::sender::send_ping;
use std::env;

///Main function to parse argument and send a single ping.
fn main() {

    // ----------------------- arg parsing -----------------------
    // Define the CLI using clap
    let matches = Command::new("rs_ping")
        .version("0.1.0")
        .author("Your Name rszylkowski1@gmail.com")
        .about("A Rust-based ICMP packet sender. Gives ability to use multithreading for faster packet sending.
        
        REQUIRES sudo/Admin rights to run.")
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Number of packets to send")
                .default_value(config::DEFAULT_PACKET_COUNT_STR),
        )
        .arg(
            Arg::new("interface")
                .short('I')
                .long("interface")
                .value_name("INTERFACE")
                .help("Interface to send packets from"),
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
            Arg::new("faf")
                .short('f')
                .long("faf")
                .help("Enable multithreading (fast as f***)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("thread")
                .short('t')
                .long("thread")
                .value_name("THREAD")
                .help("Number of threads to use (requires --faf)")
                .default_value(config::DEFAULT_THREADS_STR),
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

    // Print parsed arguments (for debugging)
    if debug {
        println!("Debugging enabled");
        println!("Count: {}", count);
        println!("Interval: {} ms", interval);
        println!("Target: {}", target);
        println!("No Wait: {}", no_wait);
        println!("Delay: {} ms", delay);
    }

    // ################### arg parsing ###################

    // ----------------------- Delay Before Start -----------------------
    if delay > 0 {
        println!("Delaying for {} ms before starting...", delay);
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
    //Send a single ping
    match send_ping(target) {
        Ok(_) => println!("Ping set sucessfuly to {target}"),
        Err(e) => eprintln!("Failed to send ping: {e}"),
    }
}