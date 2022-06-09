use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct AppArgs {
    /// Maximum value for the d parameter in us
    /// The algorithm is executed from 1 to d_max.
    #[clap(short)]
    d_max: u64,

    /// Total duration in us
    #[clap(short)]
    t: u64,

    /// number of times to execute the function for each pair (t, d)
    #[clap(short, long)]
    runs: usize,

    /// File to save the csv output
    #[clap(short, long)]
    output_filename: String,
}

impl AppArgs {
    pub fn new() -> AppArgs {
        AppArgs::parse()
    }

    pub fn d_max(&self) -> u64 {
        self.d_max
    }

    pub fn t(&self) -> u64 {
        self.t
    }

    pub fn runs(&self) -> usize {
        self.runs
    }

    pub fn output_filename(&self) -> &str {
        &self.output_filename
    }
}
