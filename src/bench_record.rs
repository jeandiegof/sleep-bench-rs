use pinscher::{CpuTimeBencher, EnergyBencher};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BenchRecord {
    /// d parameter of the algorithm
    d: u64,

    /// CPU active time in micro seconds
    cpu_time_us: u128,

    /// Package energy in micro Joules
    package_energy: u64,

    /// Core energy in micro Joules
    core_energy: u64,
}

impl BenchRecord {
    pub fn new(d: u64, cpu_time: CpuTimeBencher, energy: EnergyBencher) -> Self {
        let cpu_time_us = cpu_time.cpu_time().unwrap().as_micros();
        let package_energy = energy.package_energy();
        let core_energy = energy.core_energy();

        Self {
            d,
            cpu_time_us,
            package_energy,
            core_energy,
        }
    }
}
