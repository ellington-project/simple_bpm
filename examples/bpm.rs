extern crate hodges;
use hodges::*;
extern crate simple_bpm;
use simple_bpm::SimpleEstimator;

use std::env;

/*
    Calculate the bpm/tempo of an audio file using the naive estimator and hodges.
    Example usage:
        bpm <audiofile>
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    println!("\nReading from file: {}", filename);

    let mut estimator = SimpleEstimator::default();

    let state: State<&[f32]> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");
    let bpm = estimator.analyse(state.flatten().cloned());

    println!("Calculated bpm: {}", bpm);
}
