#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::ParameterizedBenchmark;

extern crate simple_bpm;
use simple_bpm::SimpleEstimator;

extern crate hodges;

use hodges::State;

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Track {
    Bess,
    Blues,
    Campus,
    Rabbits,
    Rent,
    Squatty,
    Stars,
    Things,
}

static TRACKS: [Track; 8] = [
    Track::Bess,
    Track::Blues,
    Track::Campus,
    Track::Rabbits,
    Track::Rent,
    Track::Squatty,
    Track::Stars,
    Track::Things,
];

impl Track {
    pub fn to_mp3(&self) -> PathBuf {
        PathBuf::from(match self {
            Track::Bess => "audio/tracks/bess.mp3",
            Track::Blues => "audio/tracks/blues.mp3",
            Track::Campus => "audio/tracks/campus.mp3",
            Track::Rabbits => "audio/tracks/rabbits.mp3",
            Track::Rent => "audio/tracks/rent.mp3",
            Track::Squatty => "audio/tracks/squatty.mp3",
            Track::Stars => "audio/tracks/stars.mp3",
            Track::Things => "audio/tracks/things.mp3",
        })
    }
}

struct Data {
    audio_d: Option<std::collections::HashMap<Track, Vec<f32>>>,
}

impl Data {
    pub fn init(&mut self) {
        // Only initialise the hashmap if it's empty.
        if let None = self.audio_d {
            println!("Data unavailable, initialising.");
            self.audio_d = Some(HashMap::new());
            println!("Running make");
            let output = Command::new("sh")
                .arg("-c")
                .arg("cd audio; make -j32")
                .output()
                .expect("failed to execute process");
            println!("status: {}", output.status);
            // iterate over the tracks, and load them.
            for t in TRACKS.iter() {
                println!("Reading audio: {:?}", t.to_mp3());
                let filename = t.to_mp3();
                let state: State<&[f32]> =
                    State::from_file(filename.clone()).expect("Failed to open file with libhodges");
                let samples: Vec<f32> = state.flatten().cloned().collect();
                match &mut self.audio_d {
                    Some(hm) => hm.insert(*t, samples),
                    None => panic!("This should have been initialised!"),
                };
            }
        } else {
            println!("Data already initialised.");
        }
    }

    pub fn get(&self, t: &Track) -> &Vec<f32> {
        match &self.audio_d {
            Some(hm) => hm.get(t).expect("Couldn't find track in map!"),
            None => panic!("This should have been initialised!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Accuracy {
    Fast,
    Medium,
    Slow,
}

impl Accuracy {
    fn as_arg(&self) -> u32 {
        match self {
            Accuracy::Fast => 0,
            Accuracy::Medium => 1,
            Accuracy::Slow => 2,
        }
    }
}

static mut DATA: Data = Data { audio_d: None };

fn bench_bess(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Bess)
    };

    c.bench(
        "bess",
        ParameterizedBenchmark::new(
            "bess",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_blues(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Blues)
    };

    c.bench(
        "blues",
        ParameterizedBenchmark::new(
            "blues",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_campus(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Campus)
    };

    c.bench(
        "campus",
        ParameterizedBenchmark::new(
            "campus",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_rabbits(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Rabbits)
    };

    c.bench(
        "rabbits",
        ParameterizedBenchmark::new(
            "rabbits",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_rent(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Rent)
    };

    c.bench(
        "rent",
        ParameterizedBenchmark::new(
            "rent",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_squatty(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Squatty)
    };

    c.bench(
        "squatty",
        ParameterizedBenchmark::new(
            "squatty",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_stars(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Stars)
    };

    c.bench(
        "stars",
        ParameterizedBenchmark::new(
            "stars",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_things(c: &mut Criterion) {
    let data = unsafe {
        DATA.init();
        DATA.get(&Track::Things)
    };

    c.bench(
        "things",
        ParameterizedBenchmark::new(
            "things",
            move |b, a| {
                b.iter(|| SimpleEstimator::with_accuracy(a.as_arg()).analyse(data.iter().cloned()))
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        ),
    );
}

fn bench_estimator_default(c: &mut Criterion) {
    unsafe { DATA.init() }
    let mut estimator = SimpleEstimator::default();

    c.bench(
        "default",
        ParameterizedBenchmark::new(
            "default",
            move |b, p| b.iter(|| unsafe { estimator.analyse(DATA.get(p).iter().cloned()) }),
            TRACKS.iter(),
        ),
    );
}

fn bench_estimator_fast(c: &mut Criterion) {
    unsafe { DATA.init() }
    let mut estimator = SimpleEstimator::with_accuracy(0);

    c.bench(
        "fast-high-error",
        ParameterizedBenchmark::new(
            "fast-high-error",
            move |b, p| b.iter(|| unsafe { estimator.analyse(DATA.get(p).iter().cloned()) }),
            TRACKS.iter(),
        ),
    );
}

fn bench_estimator_medium(c: &mut Criterion) {
    unsafe { DATA.init() }
    let mut estimator = SimpleEstimator::with_accuracy(1);

    c.bench(
        "medium-medium-error",
        ParameterizedBenchmark::new(
            "medium-medium-error",
            move |b, p| b.iter(|| unsafe { estimator.analyse(DATA.get(p).iter().cloned()) }),
            TRACKS.iter(),
        ),
    );
}

fn bench_estimator_slow(c: &mut Criterion) {
    unsafe { DATA.init() }
    let mut estimator = SimpleEstimator::with_accuracy(2);

    c.bench(
        "slow-low-error",
        ParameterizedBenchmark::new(
            "slow-low-error",
            move |b, p| b.iter(|| unsafe { estimator.analyse(DATA.get(p).iter().cloned()) }),
            TRACKS.iter(),
        ),
    );
}

criterion_group!(
    benches,
    bench_bess,
    bench_blues,
    bench_campus,
    bench_rabbits,
    bench_rent,
    bench_squatty,
    bench_stars,
    bench_things,
    bench_estimator_default,
    bench_estimator_fast,
    bench_estimator_medium,
    bench_estimator_slow,
);
criterion_main!(benches);
