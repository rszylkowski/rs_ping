# rs_ping

`rs_ping` is a Rust-based ICMP packet sender designed for testing network configurations and firewalls. It allows for high-frequency ICMP packet sending with configurable parameters such as the number of packets, interval time, multithreading, and report generation.

## Features
- Send ICMP packets to a specified IP address.
- Configurable parameters for packet count, interval, and threading.
- Optional CSV report generation.
- Cross-platform support (Unix and Windows).

## Requirements
- Rust (latest stable version)
- Administrator/root privileges (required for raw socket operations)

## Compilation
To compile the project, ensure you have Rust installed. Then run:
```bash
cargo build --release
```

## Usage
```bash
sudo ./rs_ping <options> <ip>
```

## Parameters

* -c <count>: Number of packets to send (default: 4).
* -I <interface>: Interface to send packets from (not requred -> default on linux and BSD systems rs_ping uses default routing interface.
* -i <interval>: Interval between packets in milliseconds (default 1000ms, minimal value 2ms).
* --faf: Enable multithreading (interval is ignored when this flag is present).
* --thread <count>: Number of threads to use (default: 1, max: 8; used only with --faf and -c).
* --raport: Generate a CSV report of the run.
* -n: Do not wait for a response.
* -h: Display help.

## Example

Send 10 packets to 192.168.1.1 with a 10ms interval:
```bash
sudo ./rs_ping -c 10 -t 10 192.168.1.1
```

Enable multithreading with 4 threads and generate a report:
```bash
sudo ./rs_ping --faf --thread 4 --raport 192.168.1.1
```