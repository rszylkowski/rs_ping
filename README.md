# rs_ping

## Warning ⚠️

This tool is intended for educational and testing purposes only. Use it responsibly and only on systems or networks you own or have explicit permission to test. Unauthorized use of this tool on external systems or networks may be considered illegal and could result in severe consequences.

**You are using this tool at your own risk. The authors and contributors are not responsible for any misuse or damage caused by this tool.**

By downloading and using this tool, you acknowledge that you have been informed about the potential risks and agree to use it responsibly.

## Description
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

This tool is currently under development. The list of features reflects the functionality available in the current release. Current version: **0.1.0**.


## Requirements
- Rust (latest stable version)
- Administrator/root privileges (required for raw socket operations)
- config.tomp for cross-compilation is added to repository under <project>/.cargo/config.toml
## Cross-Compiling for Windows

To cross-compile this project for the `x86_64-pc-windows-gnu` target, you need to install the NPcap SDK and configure your environment.

### Steps to Install the NPcap SDK

1. **Download the NPcap SDK**  
   Visit the [NPcap SDK download page](https://npcap.com/#download) and download the SDK ZIP file.

2. **Extract the SDK**  
   Extract the downloaded ZIP file to a directory on your system.

3. **Locate the `Packet.lib` File**  
   Inside the extracted SDK, navigate to the `Lib\X64` directory and locate the `Packet.lib` file.

4. **Copy the `Packet.lib` File**  
   Copy the `Packet.lib` file to the appropriate directory for your cross-compilation toolchain:
   ```bash
   sudo cp /path/to/Packet.lib /usr/x86_64-w64-mingw32/lib/
   ```
5. **Update the Cargo Configuration**
    ``` bash
    [target.x86_64-pc-windows-gnu]
    linker = "x86_64-w64-mingw32-gcc"
    rustflags = [
        "-L/usr/x86_64-w64-mingw32/lib" # Path to Packet.lib
    ]
    ```
6. **Install the Rust Target**
    ```bash
    rustup target add x86_64-pc-windows-gnu
    ```
7. **Download mingw-w64**
    ```bash
    sudo apt install gcc-mingw-w64-x86-64
    ```
8. **Build the Project**
    ```bash
    cargo build --release --target x86_64-pc-windows-gnu
    ```

## Cross-Compiling for Raspberry Pi

To compile this project for a Raspberry Pi running Ubuntu OS, you need to ensure the correct target architecture and toolchain are installed.

### Steps for Cross-Compilation

1. **Install the Rust Target for Raspberry Pi**  
   Depending on your Raspberry Pi's architecture, install the appropriate Rust target:
   ```bash
   rustup target add armv7-unknown-linux-gnueabihf  # For ARMv7 (32-bit)
   rustup target add aarch64-unknown-linux-gnu      # For ARMv8 (64-bit)
   ```
2. **Install the Cross-Compilation Toolchain**
    ```bash
    sudo apt install gcc-arm-linux-gnueabihf  # For ARMv7
    sudo apt install gcc-aarch64-linux-gnu    # For ARMv8
    ```
3. **Update the Cargo Configuration**
    ```bash
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"

    [target.aarch64-unknown-linux-gnu]
    linker = "aarch64-linux-gnu-gcc"
    ```
4. **Build the Project for Raspberry Pi**
    ```bash
    cargo build --release --target armv7-unknown-linux-gnueabihf  # For ARMv7
    cargo build --release --target aarch64-unknown-linux-gnu      # For ARMv8
    ```




## Compilation WSL/Linux
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
This command sends 100 packets to 127.0.0.1, a 500ms interval is not used when FaF is enabled, using 4 threads, and generates a CSV report.

## CSV Report Format
The CSV report includes the following columns:


   Thread ID   |   Packet Number   |   Timestamp   |	Target  |   Status  |   Error   
----------- | ------------------ | -------------- | ---------- | ---------- | ---------
|  1 |	1   |	2025-03-20 00:56:44.199 |	127.0.0.1   |	Success |	 |
|  2 |	1   |	2025-03-20 00:56:44.199 |	127.0.0.1   |	Success |	 |
|   ... |	... |	... |	... |	... |	... | 

