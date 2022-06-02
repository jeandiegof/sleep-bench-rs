mod app_args;
mod bench_record;

use app_args::AppArgs;
use bench_record::BenchRecord;
use csv::Writer;
use pinscher::Bencher;
use std::{thread, time::Duration};

fn sleep(t: u64, d: u64) {
    for _ in 0..(t / d) {
        thread::sleep(Duration::from_millis(d));
    }
}

fn main() {
    let args = AppArgs::new();
    let mut csv_writer = Writer::from_path(args.output_filename()).unwrap();

    for d in 1..args.d_max() {
        let bench_result = Bencher::bench(|| sleep(args.t(), d));
        let record = BenchRecord::new(d, bench_result);
        println!("{:?}", record);

        csv_writer.serialize(record).unwrap();
        csv_writer.flush().unwrap();
    }
}
