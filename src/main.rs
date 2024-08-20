use std::thread;
use std::time::Duration;
use clap::Parser;
use ulid::Ulid;

#[derive(Parser, Debug)]
#[command(author = "Yutak <yutak03.dev@gmail.com>", version = "0.1.0", about = "ULID Generator")]
struct Args {
    /// Number of ULIDs to generate
    /// If 0 is provided, 1 ULID will be generated
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    /// Number of interval generating ULID
    /// Unit is milliseconds
    /// If 0 is provided, interval will be 100 milliseconds.
    #[arg(short, long, default_value_t = 100)]
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