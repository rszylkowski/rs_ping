# rs_ping

`rs_ping` is a Rust-based ICMP packet sender. It allows you to send ICMP echo requests (ping) to a target IP address and supports multithreading for improved performance. The tool also generates detailed CSV reports for analysis.

## Features
- **Multithreading Support**: Use the `--faf` flag to enable multithreading and specify the number of threads with `--threads`.
- **CSV Report Generation**: Generate a detailed CSV report with the `--report` flag. The report includes:
  - **Thread ID**: Indicates which thread handled each packet.
  - **Packet Number**: Maintains the order of packets sent by each thread.
  - **Timestamp**: Shows when each packet was sent.
  - **Target**: The target IP address.
  - **Status**: Whether the packet was sent successfully or failed.
  - **Error**: Any error encountered while sending the packet.
- **Customizable Options**: Configure the number of packets, interval between packets, and delay before starting.

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
sudo rs_ping [OPTIONS] <target>
```

## Arguments
  <target>  Target IP address

## Parameters [OPTIONS]

* -c, --count <COUNT>        Number of packets to send [default: 4]
* -i, --interval <INTERVAL>  Interval between packets in milliseconds [default: 1000]
* -d, --delay <DELAY>        Delay before starting to send packets in milliseconds [default: 0]
* -D, --debug                Enable debug output
* -r, --report               Generate a CSV report
* -n, --no-wait              Do not wait for a response
* -f, --faf                  Enable multithreading
* -t, --threads <THREADS>    Number of threads to use (requires --faf) [default: 1]
* -h, --help                 Print help
* -V, --version              Print version

## Usage

### Basic Usage

```bash
sudo ./rs_ping --count 10 --interval 1000 127.0.0.1
```

### Multithreading

Enable multithreading with the --faf flag and specify the number of threads with --threads.
``` bash
sudo ./rs_ping --count 100 --interval 500 --faf --threads 4 127.0.0.1
```

### Generate a CSV Report
Use the --report flag to generate a CSV report.
``` bash
sudo ./rs_ping --count 50 --interval 200 --report 127.0.0.1
```
The report will be saved with a timestamped filename, e.g., rs_ping_report_YYYY-MM-DD_HH-MM-SS.csv.

### Full Example
``` bash
sudo ./rs_ping --count 100 --interval 500 --faf --threads 4 --report 127.0.0.1
```
This command sends 100 packets to 127.0.0.1 with a 500ms interval, using 4 threads, and generates a CSV report.

## CSV Report Format
The CSV report includes the following columns:


Thread ID   |   Packet Number   |   Timestamp   |	Target  |   Status  |   Error   |
-----------  ------------------  --------------  ----------  ----------  ----------
1 |	1   |	2025-03-20 00:56:44.199 |	127.0.0.1   |	Success |	
-----------  ------------------  --------------  ----------  ----------  ----------
2 |	1   |	2025-03-20 00:56:44.199 |	127.0.0.1   |	Success |	
-----------  ------------------  --------------  ----------  ----------  ----------
... |	... |	... |	... |	... |	... |

