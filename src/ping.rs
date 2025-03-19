// filepath: [ping.rs](http://_vscodecontentref_/6)
use chrono::Local;
use std::thread;
use std::time::Duration;

use crate::icmp::sender::send_ping;
use crate::report::Report;

/// Sends multiple pings based on the provided arguments.
///
/// # Arguments
/// * [count](http://_vscodecontentref_/7) - Number of packets to send.
/// * [interval](http://_vscodecontentref_/8) - Interval between packets in milliseconds.
/// * [delay](http://_vscodecontentref_/9) - Delay before starting in milliseconds.
/// * [target](http://_vscodecontentref_/10) - Target IP address.
/// * [no_wait](http://_vscodecontentref_/11) - Whether to wait for a response.
/// * [debug](http://_vscodecontentref_/12) - Whether to enable debug output.
/// * [report](http://_vscodecontentref_/13) - Whether to generate a CSV report.
pub fn send_multiple_pings(
    count: u32,
    interval: u64,
    delay: u64,
    target: &str,
    no_wait: bool,
    debug: bool,
    report: bool,
) {
    // ----------------------- Delay Before Start -----------------------
    if delay > 0 {
        println!("Delaying for {} ms before starting...", delay);
        thread::sleep(Duration::from_millis(delay));
    }

    // ----------------------- CSV Report Setup -----------------------
    let mut report_file = if report { Some(Report::new()) } else { None };

    // ----------------------- Send Multiple Pings -----------------------
    for i in 1..=count {
        let mut timestamp = String::new();
        if debug || report {
            let now = Local::now();
            timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(); // Includes milliseconds
            let nanos = now.timestamp_subsec_nanos() % 1_000_000;
            timestamp = format!("{}.{}", timestamp, nanos);
        }

        let mut status = "Success";
        let mut error_message = String::new();

        match send_ping(target) {
            Ok(_) => {
                if debug {
                    println!("Packet {}/{} sent successfully to {} at {}", i, count, target, timestamp);
                }
            }
            Err(e) => {
                status = "Failed";
                error_message = e.to_string();
                if debug {
                    eprintln!("Failed to send packet {}/{}: {} at {}", i, count, e, timestamp);
                }
            }
        }

        // Write to the CSV report if enabled
        if let Some(report) = report_file.as_mut() {
            report.write_row(i, &timestamp, target, status, &error_message);
        }

        // Delay between packets
        if i < count {
            thread::sleep(Duration::from_millis(interval));
        }
    }

    println!("Finished sending {} packets to {}", count, target);
    if let Some(report) = report_file {
        report.finalize();
    }
}