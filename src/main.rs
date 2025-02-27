use clap::Parser;
use std::thread;
use std::time::Duration;
use ulid::Ulid;

#[derive(Parser, Debug)]
#[command(
    author = "Yutak <yutak03.dev@gmail.com>",
    version = "0.1.0",
    about = "ULID Generator"
)]
struct Args {
    /// Number of ULIDs to generate
    #[arg(short = 'C', long, default_value_t = 1)]
    count: u32,

    /// Interval between ULID generation in milliseconds
    #[arg(short = 'I', long, default_value_t = 100)]
    interval: u64,

    /// Generate nil ULID (all zeros)
    #[arg(short = 'N', long = "nil", default_value_t = false)]
    nil: bool,
}

/// Generate a single ULID based on the nil flag
fn generate_ulid(nil: bool) -> Ulid {
    if nil {
        Ulid::nil()
    } else {
        Ulid::new()
    }
}

/// Print a formatted list of ULIDs
fn print_ulids(count: u32, interval: u64, nil: bool) {
    println!("Generated ULIDs:");
    println!("{}", "─".repeat(50));

    for i in 0..count {
        let id = generate_ulid(nil);
        println!(" {}) {}", i + 1, id);

        // If multiple ULIDs are created, insert an interval between them.
        if i < count - 1 {
            thread::sleep(Duration::from_millis(interval));
        }
    }

    println!("{}", "─".repeat(50));
}

fn main() {
    let args = Args::parse();

    // Use the actual count value directly (the default is already 1)
    print_ulids(args.count, args.interval, args.nil);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{Duration, Instant};

    /// Test for default argument values
    #[test]
    fn test_args_default_values() {
        let args = Args::parse_from(&["test"]);
        assert_eq!(args.count, 1);
        assert_eq!(args.interval, 100);
        assert_eq!(args.nil, false);
    }

    /// Test for custom argument values
    #[test]
    fn test_args_custom_values() {
        let args = Args::parse_from(&["test", "-C", "5", "-I", "200", "-N"]);
        assert_eq!(args.count, 5);
        assert_eq!(args.interval, 200);
        assert_eq!(args.nil, true);
    }

    /// Test ULID generation functions
    #[test]
    fn test_generate_ulid() {
        // Test regular ULID
        let regular = generate_ulid(false);
        assert_ne!(regular, Ulid::nil());

        // Test nil ULID
        let nil = generate_ulid(true);
        assert_eq!(nil, Ulid::nil());
    }

    /// Test printing multiple ULIDs with intervals
    #[test]
    fn test_ulid_generation_with_interval() {
        let count = 3;
        let interval = 50;
        let start = Instant::now();

        // Capture stdout for testing
        let mut output = Vec::new();
        {
            // Generate ULIDs and measure time
            for i in 0..count {
                let id = generate_ulid(false);
                writeln!(output, "{}", id).unwrap();

                if i < count - 1 {
                    thread::sleep(Duration::from_millis(interval));
                }
            }
        }

        let duration = start.elapsed();
        let output = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output.lines().collect();

        // Verify correct count
        assert_eq!(lines.len(), 3);

        // Verify timing (at least 2 intervals but not too long)
        assert!(duration >= Duration::from_millis(100));
        assert!(duration < Duration::from_millis(200));

        // Verify each line is a valid ULID
        for line in lines {
            assert!(Ulid::from_string(line).is_ok());
        }
    }
}
