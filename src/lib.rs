// pure rust implementation of a bpm analysis algorithm
// derived from Mark Hill's implementation, available
// at http://www.pogo.org.uk/~mark/bpm-tools/ contact
// mark@xwax.org for more information.

use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use itertools_num::linspace;

use itertools::iproduct;

use std::f32;

#[derive(Debug)]
pub struct SimpleEstimator {
    // tunable parameters for the algorithm.
    pub lower: f32,
    pub upper: f32,
    pub interval: u64,
    pub rate: f32,
    pub steps: u32,
    pub samples: u32,
    rng: ThreadRng,
}

impl SimpleEstimator {
    // A reasonably performing, reasonably accurate default
    pub fn default() -> SimpleEstimator {
        SimpleEstimator {
            lower: 50.0,
            upper: 450.0,
            interval: 64,
            rate: 44100.0,
            steps: 800,
            samples: 1024,
            rng: thread_rng(),
        }
    }

    // allow the user to select one of a few reasonable estimators
    // the higher accuracy, the slower the estimator
    // The accuracies roughly correspond to low/medium/high
    pub fn with_accuracy(level: u32) -> SimpleEstimator {
        match level {
            // Interval,Steps,Samples,MeanBpm,MeanSqErr,ErrOfMean,MinTime,MeanTime
            // 2048    ,800  ,2048   ,207.89 ,37.51    ,1.41     ,81.91  ,83.26
            0 => SimpleEstimator::with_settings(2048, 800, 2048),
            // 128     ,1600 ,2048   ,206.91 ,16.64    ,0.93     ,168.97 ,171.1
            1 => SimpleEstimator::with_settings(128, 1600, 2048),
            // 256     ,800  ,8192   ,206.14 ,4.98     ,0.55     ,325.41 ,328.31
            2 => SimpleEstimator::with_settings(256, 800, 8192),
            _ => SimpleEstimator::default(),
        }
        // match level {
        //     // Mean Squared Error, Error of mean, Mean Time
        //     // 1338.09,      5.34, 59.55
        //     0 => SimpleEstimator::with_settings(32, 800, 1024),
        //     // 1663.39, 7.14, 172.28
        //     1 => SimpleEstimator::with_settings(256, 3200, 1024),
        //     // 340.02, 1.16, 102.65
        //     2 => SimpleEstimator::with_settings(96, 800, 2048),
        //     // 339.93, 1.58, 98.23
        //     3 => SimpleEstimator::with_settings(128, 800, 2048),
        //     // 1002.36, 4.05, 87.05
        //     4 => SimpleEstimator::with_settings(512, 800, 2048),
        //     // 8.92, 0.11, 173.27
        //     5 => SimpleEstimator::with_settings(256, 1600, 2048),
        //     // 338.11, 1.31, 442.56
        //     6 => SimpleEstimator::with_settings(32, 3200, 2048),
        //     // 7.36, 0.13, 339.88
        //     7 => SimpleEstimator::with_settings(512, 3200, 2048),
        //     // 5.84, 0.06, 198.03
        //     8 => SimpleEstimator::with_settings(96, 800, 4096),
        //     _ => SimpleEstimator::default(),
        // }
    }

    pub fn with_settings(interval: u64, steps: u32, samples: u32) -> SimpleEstimator {
        SimpleEstimator {
            lower: 50.0,
            upper: 450.0,
            interval: interval,
            rate: 44100.0,
            steps: steps,
            samples: samples,
            rng: thread_rng(),
        }
    }

    /*
     * main analysis function
     * We currently have the fairly major (imho) limitation that the entire
     * vector of samples must be read into memory before we can process it.
     */
    pub fn analyse<T>(self: &mut SimpleEstimator, samples: T) -> f32
    where
        T: Iterator<Item = f32>,
    {
        /* Maintain an energy meter (similar to PPM), and
         * at regular intervals, sample the energy to give a
         * low-resolution overview of the track
         */
        let nrg: Vec<f32> = samples
            .scan(0.0, |v, s| {
                let z: f32 = s.abs();

                // Rewrite an explicit if statement as multiplication by the condition
                // i.e. if cnd == true, then:
                //    v + (1 * ...) - (0 * ...)
                //  else
                //    v + (0 * ...) - (1 * ...)
                // This improves performance a little by avoiding a cmp instruction,
                // which would potentially cause a pipeline stall if it mispredicts
                let cnd = (z > *v) as i32 as f32;
                Some(*v + (cnd * (z - *v) / 8.0) - (cnd * (*v - z) / 512.0))
            })
            .step_by(self.interval as usize)
            .collect();

        self.scan_for_bpm(&nrg)
    }

    /*
     * Scan a range of BPM values for the one with the
     * minimum autodifference
     */
    #[inline(always)]
    fn scan_for_bpm(self: &mut SimpleEstimator, nrg: &Vec<f32>) -> f32 {
        let slowest = self.bpm_to_interval(self.lower);
        let fastest = self.bpm_to_interval(self.upper);

        // get the length of nrg as a float, so that we don't keep converting it
        let flen = nrg.len() as f32;

        // until we can generate random numbers, use the mean of the uniform distribution over [0.0, 1.0]
        let udistr = Uniform::new(0.0, flen);

        // rust won't let us iterate over floats :(
        // use the linspace method from itertools-num to create a range, and save it to a vector for performance.
        let intervals: Vec<f32> = linspace::<f32>(slowest, fastest, self.steps as usize).collect();

        // collect the random samples into a vector so we don't block waiting on the rng when we want to be performing autodifferences
        let randsamples: Vec<Vec<f32>> = (0..self.steps)
            .map(|_| {
                self.rng
                    .sample_iter(&udistr)
                    .take(self.samples as usize)
                    .collect()
            })
            .collect();

        let (_, trough) = intervals.into_iter().zip(randsamples).fold(
            (f32::INFINITY, f32::NAN),
            |(height, trough), (interval, rsamples)| {
                // .for_each(|(interval, rsamples)| {
                // Iterate over the samples, and use each as the midpoint of our autodifference method
                let t = rsamples.into_iter().fold(0.0, |acc, mid| {
                    acc + utils::autodifference(&nrg, flen, interval, mid)
                });

                if t < height {
                    (t, interval)
                } else {
                    (height, trough)
                }
            },
        );

        self.interval_to_bpm(trough)
    }

    /*
     * Beats-per-minute to a sampling interval in energy space
     */
    #[inline(always)]
    fn bpm_to_interval(self: &SimpleEstimator, bpm: f32) -> f32 {
        let beats_per_second: f32 = bpm / 60.0;
        let samples_per_beat: f32 = self.rate / beats_per_second;
        samples_per_beat / self.interval as f32
    }

    /*
     * Sampling interval in enery space to beats-per-minute
     */
    #[inline(always)]
    fn interval_to_bpm(self: &SimpleEstimator, interval: f32) -> f32 {
        let samples_per_beat: f32 = interval * self.interval as f32;
        let beats_per_second: f32 = self.rate / samples_per_beat;
        beats_per_second * 60.0
    }
}
mod utils {

    /*
     * Test an autodifference for the given interval
     */
    #[inline]
    pub fn autodifference(nrg: &Vec<f32>, flen: f32, interval: f32, mid: f32) -> f32 {
        // define some arrays of constants
        const BEATS: [f32; 12] = [
            -32.0, -16.0, -8.0, -4.0, -2.0, -1.0, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0,
        ];
        const NOBEATS: [f32; 4] = [-0.5, -0.25, 0.25, 0.5];

        let v: f32 = sample(&nrg, flen, mid);

        let (bd, bt) = BEATS.iter().fold((0.0, 0.0), |(d, t), b| {
            let y: f32 = sample(&nrg, flen, mid + b * interval);
            let w = 1.0 / b.abs();

            (d + w * (y - v).abs(), t + w)
        });

        let (nd, nt) = NOBEATS.iter().fold((0.0, 0.0), |(d, t), b| {
            let y = sample(&nrg, flen, mid + b * interval);
            let w = b.abs();

            (d - w * (y - v).abs(), t + w)
        });

        (bd + nd) / (bt + nt)
    }

    /*
     * Sample from the metered energy
     *
     * No need to interpolate and it makes a tiny amount of difference; we
     * take a random sample of samples, any errors are averaged out.
     */
    #[inline]
    fn sample(nrg: &Vec<f32>, flen: f32, offset: f32) -> f32 {
        let n: f32 = offset.floor();
        let i: usize = n as usize; // does this do (in c terms) `i = (u32) n`?

        if n >= 0.0 && n < flen {
            nrg[i]
        } else {
            0.0
        }
    }

}
