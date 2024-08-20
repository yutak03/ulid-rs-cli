use std::thread;
use std::time::Duration;
use clap::Parser;
use ulid::Ulid;

#[derive(Parser, Debug)]
#[command(author = "Yutak <yutak03.dev@gmail.com>", version = "0.1.0", about = "ULID Generator")]
struct Args {
    /// Number of ULIDs to generate
    /// If 0 is provided, 1 ULID will be generated
    #[arg(short = 'C', long, default_value_t = 1)]
    count: u32,

    /// Number of interval generating ULID
    /// Unit is milliseconds
    /// If 0 is provided, interval will be 100 milliseconds.
    #[arg(short = 'I', long, default_value_t = 100)]
    interval: u64
}

fn main() {
    let args = Args::parse();
    let count = if args.count == 0 { 1 } else { args.count };

    for i in 0..count {
        let id = Ulid::new();
        println!("{}", id);

        if i < count - 1 {
            thread::sleep(Duration::from_millis(args.interval));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Instant, Duration};
    use std::io::{self, Write};

    /// Test to set defalut value
    #[test]
    fn test_args_default_values() {
        let args = Args::parse_from(&["test"]);
        assert_eq!(args.count, 1);
        assert_eq!(args.interval, 100);
    }

    /// Test to set custom value by args
    #[test]
    fn test_args_custom_values() {
        let args = Args::parse_from(&["test", "-C", "5", "-I", "200"]);
        assert_eq!(args.count, 5);
        assert_eq!(args.interval, 200);
    }

    /// Tests the basic functionality of ULID generation and intervals between generations
    #[test]
    fn test_ulid_generation() {
        let args = Args { count: 3, interval: 50 };
        let start = Instant::now();

        // Capture stdout
        let mut output = Vec::new();
        {
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            for i in 0..args.count {
                let id = Ulid::new();
                writeln!(output, "{}", id).unwrap();

                if i < args.count - 1 {
                    thread::sleep(Duration::from_millis(args.interval));
                }
            }

            handle.write_all(&output).unwrap();
        }

        let duration = start.elapsed();
        let output = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output.lines().collect();

        assert_eq!(lines.len(), 3);
        assert!(duration >= Duration::from_millis(100)); // At least 2 intervals of 50ms
        assert!(duration < Duration::from_millis(200)); // But not too long

        // Verify that each line is a valid ULID
        for line in lines {
            assert!(Ulid::from_string(line).is_ok());
        }
    }
}