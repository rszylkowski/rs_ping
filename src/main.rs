// filepath: [main.rs](http://_vscodecontentref_/0)
//! Entry point for the rs_ping application.

mod icmp;
mod config;

use icmp::sender::send_ping;
use std::env;

///Main function to parse argument and send a single ping.
fn main() {
    //Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rd_ping <IP_ADDRESS>");
        std::process::exit(1);
    }

    let target_ip = &args[1];

    //Send a single ping
    match send_ping(target_ip) {
        Ok(_) => println!("Ping set sucessfuly to {target_ip}"),
        Err(e) => eprintln!("Failed to send ping: {e}"),
    }
}