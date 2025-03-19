use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

/// Represents a CSV report for the ping results.
pub struct Report {
    file_name: String,
    file: Option<std::fs::File>,
}

impl Report {
    /// Creates a new report file with a timestamped name.
    pub fn new() -> Self {
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
        let file_name = format!("rs_ping_report_{}.csv", timestamp);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_name)
            .expect("Failed to create report file");

        writeln!(file, "Thread ID,Packet Number,Timestamp,Target,Status,Error")
            .expect("Failed to write CSV header");

        println!("Report will be saved to {}", file_name);

        Self {
            file_name,
            file: Some(file),
        }
    }

    /// Writes a single row to the report.
    pub fn write_row(
        &mut self,
        thread_id: u32,
        packet_number: u32,
        timestamp: &str,
        target: &str,
        status: &str,
        error_message: &str,
    ) {
        if let Some(file) = self.file.as_mut() {
            writeln!(
                file,
                "{},{},{},{},{},{}",
                thread_id, packet_number, timestamp, target, status, error_message
            )
            .expect("Failed to write to report file");
        }
    }

    /// Finalizes the report and closes the file.
    pub fn finalize(self) {
        println!("Report saved successfully to {}", self.file_name);
    }
}