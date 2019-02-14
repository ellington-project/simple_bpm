extern crate simple_bpm;
use simple_bpm::SimpleEstimator;

extern crate hodges;

use hodges::State;

use std::env;

use std::time::Instant;

struct EstimatorStat {
    // Estimator settings
    interval: u64,
    stepc: u32,
    samplec: u32,
    // Results
    mean_bpm: f32,
    mean_time: f32,
    min_time: f32,
    mean_sq_err: f32,
    err_of_mean: f32,
}

fn run_trial(
    estimator: &mut SimpleEstimator,
    audiosamples: &Vec<f32>,
    expected: f32,
    trials: i32,
    interval: u64,
    stepc: u32,
    samplec: u32,
) -> Option<EstimatorStat> {
    // run the estimator 5 times, to see if we get a good MSE or time
    let pretrials = 10;
    let (total_time, total_err_sqs) = (0..pretrials).fold((0, 0.0), |(ta, ea), _| {
        let now = Instant::now();
        let b = estimator.analyse(audiosamples.iter().cloned());
        // get a very rough elapsed time
        let t = now.elapsed().as_nanos();

        let e = (b - expected) * (b - expected);

        (ta + t, ea + e)
    });

    let mse = (total_err_sqs / pretrials as f32);
    let met = (total_time as f32 * 1e-6) / (pretrials as f32);

    if mse > 25.0 || met > 500.0 {
        eprintln!(
            "Rejecting {:5}/{:5}/{:6} -- MSE = {:9.3}, Mean Time = {:7.3}",
            interval, stepc, samplec, mse, met
        );
        return None;
    } else {
        eprintln!(
            "Accepting {:5}/{:5}/{:6} -- MSE = {:9.3}, Mean Time = {:7.3}",
            interval, stepc, samplec, mse, met
        );
    }

    let (total_bpm, total_time, min_time, total_err_sqs) =
        (0..trials).fold((0.0, 0, std::u128::MAX, 0.0), |(ba, ta, ma, ea), _| {
            let now = Instant::now();
            let b = estimator.analyse(audiosamples.iter().cloned());
            // get a very rough elapsed time
            let t = now.elapsed().as_nanos();

            let e = (b - expected) * (b - expected);

            let nm = if t < ma { t } else { ma };

            (ba + b, ta + t, nm, ea + e)
        });

    let mean_bpm = total_bpm / trials as f32;

    Some(EstimatorStat {
        interval: interval,
        stepc: stepc,
        samplec: samplec,
        mean_bpm: mean_bpm,
        mean_time: 1e-6 * total_time as f32 / trials as f32,
        min_time: min_time as f32 * 1e-6,
        mean_sq_err: total_err_sqs / trials as f32,
        err_of_mean: 100.0 * (mean_bpm - expected).abs() / expected,
    })
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let trials = args[2].parse::<i32>().unwrap();
    let expected = args[3].parse::<f32>().unwrap();

    eprintln!("\nReading from file: {}", filename);

    let state: State<&[f32]> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");

    let audiosamples: Vec<f32> = state.flatten().cloned().collect();

    // create a vector to store estimator statistics
    let mut estimator_results: Vec<EstimatorStat> = Vec::new();

    let intervals = vec![16, 32, 48, 64, 96, 128, 256, 512, 1024, 2048, 4096];
    let steps = vec![800, 1600, 3200, 6400];
    let samples = vec![1024, 2048, 4096, 8192, 16384, 32768];

    println!("Interval, Steps, Samples, MeanBpm, MeanSqErr, ErrOfMean, MinTime, MeanTime");
    for samplec in &samples {
        for stepc in &steps {
            for interval in &intervals {
                let mut estimator = SimpleEstimator::with_settings(*interval, *stepc, *samplec);

                if let Some(stat) = run_trial(
                    &mut estimator,
                    &audiosamples,
                    expected,
                    trials,
                    *interval,
                    *stepc,
                    *samplec,
                ) {
                    println!(
                        "{:8}, {:5}, {:7}, {:7.2}, {:9.2}, {:9.2}, {:7.2}, {:8.2}",
                        stat.interval,
                        stat.stepc,
                        stat.samplec,
                        stat.mean_bpm,
                        stat.mean_sq_err,
                        stat.err_of_mean,
                        stat.min_time,
                        stat.mean_time,
                    );

                    estimator_results.push(stat);
                }
            }
        }
    }

    Ok(())
}
