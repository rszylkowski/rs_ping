// filepath: [ping.rs](http://_vscodecontentref_/6)
use chrono::Local;
use std::sync::Arc;
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
/// * [faf](http://_vscodecontentref_/14) - Whether to enable FaF mode.
/// * [threads](http://_vscodecontentref_/15) - Number of threads to use (if multithreading is enabled).
pub fn send_multiple_pings(
    count: u32,
    interval: u64,
    delay: u64,
    target: &str,
    no_wait: bool,
    debug: bool,
    report: bool,
    faf: bool,
    threads: u32,
) {
    // ----------------------- Delay Before Start -----------------------
    if delay > 0 {
        println!("Delaying for {} ms before starting...", delay);
        thread::sleep(Duration::from_millis(delay));
    }

    // ----------------------- Multithreading Logic -----------------------
    let mut all_results = vec![];

    if faf {
        println!("Multithreading enabled with {} threads", threads);
        let packets_per_thread = count / threads;
        let remaining_packets = count % threads;

        let mut handles = vec![];
        let results = Arc::new(std::sync::Mutex::new(vec![]));

        for thread_id in 0..threads {
            let target = target.to_string();
            let debug = debug;
            let results = Arc::clone(&results);

            let packets_to_send = if thread_id == threads - 1 {
                packets_per_thread + remaining_packets
            } else {
                packets_per_thread
            };
            
            let handle = thread::spawn(move || {
                let mut local_results = vec![];

                for i in 1..=packets_to_send {
                    let mut timestamp = String::new();
                    if debug || report {
                        let now = Local::now();
                        timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(); // Includes milliseconds
                        let nanos = now.timestamp_subsec_nanos() % 1_000_000;
                        timestamp = format!("{}.{}", timestamp, nanos);
                    }

                    let mut status = "Success";
                    let mut error_message = String::new();

                    match send_ping(&target) {
                        Ok(_) => {
                            if debug {
                                println!("[Thread {}] Packet {}/{} sent successfully to {} at {}", thread_id + 1, i, packets_to_send, target, timestamp);
                            }
                        }
                        Err(e) => {
                        status = "Failed";
                        error_message = e.to_string();
                        if debug {
                            eprintln!("[Thread {}] Failed to send packet {}/{}: {} at {}", thread_id + 1, i, packets_to_send, e, timestamp);
                        }
                    }
                }
                // Store the results in the local vector
                local_results.push((thread_id + 1, i, timestamp, target.clone(), status, error_message));
            }

                // Push the local results to the shared results vector
                let mut shared_results = results.lock().unwrap();
                shared_results.extend(local_results);    
            });

            handles.push(handle);    
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Collect all results froma all threads
        all_results = Arc::try_unwrap(results)
            .expect("Failed to unwrap Arc")
            .into_inner()
            .expect("Failed to get inner value");
    } else {
        // ----------------------- Single Threaded Logic -----------------------
        for i in 1..=count {
            let mut timestamp = String::new();
            if debug || report {
                let now = Local::now();
                timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                let nanos = now.timestamp_subsec_nanos() % 1_000_000;
                timestamp = format!("{}.{}", timestamp, nanos);
            }

            let mut status = "Success";
            let mut error_message = String::new();

            match send_ping(&target) {
                Ok(_) => {
                    if debug {
                        println!(
                            "Packet {}/{} sent successfully to {} at {}",
                            i, count, target, timestamp
                        );
                    }
                }
                Err(e) => {
                    status = "Failed";
                    error_message = e.to_string();
                    if debug {
                        eprintln!(
                            "Failed to send packet {}/{}: {} at {}",
                            i, count, e, timestamp
                        );
                    }
                }
            }

            // Store the result locally
            all_results.push((1, i, timestamp, target.to_string(), status, error_message));

            // Delay between packets
            if i < count {
                thread::sleep(Duration::from_millis(interval));
            }
        }
    }

    // ----------------------- Sort and Generate Report -----------------------
    if report {
        all_results.sort_by(|a, b| a.2.cmp(&b.2)); // Sort by timestamp (ascending)
        let mut report_file = Report::new();
        for (thread_id, packet_number, timestamp, target, status, error_message) in all_results {
            report_file.write_row(thread_id, packet_number, &timestamp, &target, &status, &error_message);
        }
        report_file.finalize();
    }

    println!("Finished sending {} packets to {}", count, target);
}