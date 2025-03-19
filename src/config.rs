// filepath: [config.rs](http://_vscodecontentref_/4)
//! Configuration and constants for the rs_ping application.

/// Default number of packets to send.
pub const DEFAULT_PACKET_COUNT: u32 = 4;

/// Minimum allowed number of packets to send.
pub const MIN_PACKET_COUNT: u32 = 1;

/// Maximum allowed number of packets to send.
pub const MAX_PACKET_COUNT: u32 = 5000;

/// Default interval between packets in milliseconds.
pub const DEFAULT_INTERVAL_MS: u64 = 1000;

/// Minimum allowed interval between packets in milliseconds.
pub const MIN_INTERVAL_MS: u16 = 2;

/// Maximum allowed interval between packets in milliseconds.
pub const MAX_INTERVAL_MS: u64 = 10000;

/// Default buffer size for receiving packets.
pub const DEFAULT_BUFFER_SIZE: usize = 1024;



// Curentli not in use (for future development)
// /// Default packet size in bytes.
// pub const DEFAULT_PACKET_SIZE: usize = 64;

// /// Minimum allowed packet size in bytes.
// pub const MIN_PACKET_SIZE: usize = 8;

// /// Maximum allowed packet size in bytes.
// pub const MAX_PACKET_SIZE: usize = 65500;

// /// Default timeout in milliseconds.
// pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

// /// Minimum allowed timeout in milliseconds.
// pub const MIN_TIMEOUT_MS: u64 = 1000;

// /// Maximum allowed timeout in milliseconds.
// pub const MAX_TIMEOUT_MS: u64 = 60000;

// /// Default TTL (time to live) for packets.
// pub const DEFAULT_TTL: u8 = 64;

// /// Minimum allowed TTL (time to live) for packets.
// pub const MIN_TTL: u8 = 1;

// /// Maximum allowed TTL (time to live) for packets.
// pub const MAX_TTL: u8 = 255;

// /// Minimum allowed buffer size for receiving packets.
// pub const MIN_BUFFER_SIZE: usize = 512;

// /// Maximum allowed buffer size for receiving packets.
// pub const MAX_BUFFER_SIZE: usize = 65535;
