use pinscher::BenchResult;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BenchRecord {
    d: u64,
    cpu_time_us: u128,
}

impl BenchRecord {
    pub fn new(d: u64, bench_result: BenchResult) -> Self {
        let cpu_time_us = bench_result.cpu_time().as_micros();

        Self { d, cpu_time_us }
    }
}
