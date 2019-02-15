#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate simple_bpm;
use simple_bpm::SimpleEstimator;

extern crate hodges;

use hodges::State;

use std::path::PathBuf;

fn read_audiof(filename: PathBuf) -> Vec<f32> {
    let state: State<&[f32]> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");
    state.flatten().cloned().collect()
}

fn bench_estimator_default(c: &mut Criterion) {
    let samples = read_audiof(PathBuf::from(
        "/home/adam/Music/The Holophonics - Despacito.mp3",
    ));
    let mut estimator = SimpleEstimator::default();
    c.bench_function("default", move |b| {
        b.iter(|| estimator.analyse(samples.iter().cloned()))
    });
}

fn bench_estimator_fast(c: &mut Criterion) {
    /*
        Fast, high error
        This should give a mean squared error of about 9.6
    */
    let samples = read_audiof(PathBuf::from(
        "/home/adam/Music/The Holophonics - Despacito.mp3",
    ));
    let mut estimator = SimpleEstimator::with_accuracy(0);
    c.bench_function("fast-high-error", move |b| {
        b.iter(|| estimator.analyse(samples.iter().cloned()))
    });
}

fn bench_estimator_medium(c: &mut Criterion) {
    /*
        Slower, low error
        This should give a mean squared error of about 2.27
    */
    let samples = read_audiof(PathBuf::from(
        "/home/adam/Music/The Holophonics - Despacito.mp3",
    ));
    let mut estimator = SimpleEstimator::with_accuracy(1);
    c.bench_function("slow-low-error", move |b| {
        b.iter(|| estimator.analyse(samples.iter().cloned()))
    });
}

fn bench_estimator_slow(c: &mut Criterion) {
    /*
        Medium, medium error
        This should give a mean squared error of about 4.78
    */
    let samples = read_audiof(PathBuf::from(
        "/home/adam/Music/The Holophonics - Despacito.mp3",
    ));
    let mut estimator = SimpleEstimator::with_accuracy(2);
    c.bench_function("medium-medium-error", move |b| {
        b.iter(|| estimator.analyse(samples.iter().cloned()))
    });
}

criterion_group!(
    benches,
    bench_estimator_default,
    bench_estimator_fast,
    bench_estimator_medium,
    bench_estimator_slow
);
criterion_main!(benches);
