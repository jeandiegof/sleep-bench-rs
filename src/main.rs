mod app_args;
mod bench_record;

use app_args::AppArgs;
use bench_record::BenchRecord;
use csv::Writer;
use pinscher::{BenchSuite, CpuTimeBencher, EnergyBencher};
use std::{thread, time::Duration};

fn sleep(t: u64, d: u64) {
    for _ in 0..(t / d) {
        thread::sleep(Duration::from_micros(d));
    }
}

fn main() {
    let args = AppArgs::new();
    let mut csv_writer = Writer::from_path(args.output_filename()).unwrap();

    (1..=args.runs()).for_each(|r| {
        println!("Run {}/{}", r, args.runs());
        (1..=args.d_max()).for_each(|d| {
            println!("d = {}/{}, t = {}", d, args.d_max(), args.t());

            let mut cpu_time_bencher = CpuTimeBencher::new();
            BenchSuite::bench(|| sleep(args.t(), d), &mut cpu_time_bencher).unwrap();

            let mut energy_bencher = EnergyBencher::new().unwrap();
            BenchSuite::bench(|| sleep(args.t(), d), &mut energy_bencher).unwrap();

            let record = BenchRecord::new(d, cpu_time_bencher, energy_bencher);
            csv_writer.serialize(record).unwrap();
        });
    });

    csv_writer.flush().unwrap();
}
