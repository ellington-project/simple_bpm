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

fn bench_tracks(c: &mut Criterion) {
    unsafe {
        DATA.init();
    };

    c.bench(
        "tracks",
        ParameterizedBenchmark::new(
            "bess",
            move |b, a| {
                b.iter(|| unsafe {
                    SimpleEstimator::with_accuracy(a.as_arg())
                        .analyse(DATA.get(&Track::Bess).iter().cloned())
                })
            },
            vec![Accuracy::Fast, Accuracy::Medium, Accuracy::Slow],
        )
        .with_function("blues", move |b, a| {
            b.iter(|| unsafe {
                SimpleEstimator::with_accuracy(a.as_arg())
                    .analyse(DATA.get(&Track::Blues).iter().cloned())
            })
        })
        .with_function("campus", move |b, a| {
            b.iter(|| unsafe {
                SimpleEstimator::with_accuracy(a.as_arg())
                    .analyse(DATA.get(&Track::Campus).iter().cloned())
            })
        })
        .with_function("rabbits", move |b, a| {
            b.iter(|| unsafe {
                SimpleEstimator::with_accuracy(a.as_arg())
                    .analyse(DATA.get(&Track::Rabbits).iter().cloned())
            })
        })
        .with_function("rent", move |b, a| {
            b.iter(|| unsafe {
                SimpleEstimator::with_accuracy(a.as_arg())
                    .analyse(DATA.get(&Track::Rent).iter().cloned())
            })
        })
        .with_function("squatty", move |b, a| {
            b.iter(|| unsafe {
                SimpleEstimator::with_accuracy(a.as_arg())
                    .analyse(DATA.get(&Track::Squatty).iter().cloned())
            })
        })
        .with_function("stars", move |b, a| {
            b.iter(|| unsafe {
                SimpleEstimator::with_accuracy(a.as_arg())
                    .analyse(DATA.get(&Track::Stars).iter().cloned())
            })
        })
        .with_function("things", move |b, a| {
            b.iter(|| unsafe {
                SimpleEstimator::with_accuracy(a.as_arg())
                    .analyse(DATA.get(&Track::Things).iter().cloned())
            })
        }),
    );
}

fn bench_estimators(c: &mut Criterion) {
    unsafe { DATA.init() }
    let mut fast = SimpleEstimator::with_accuracy(Accuracy::Fast.as_arg());
    let mut medium = SimpleEstimator::with_accuracy(Accuracy::Medium.as_arg());
    let mut slow = SimpleEstimator::with_accuracy(Accuracy::Slow.as_arg());

    c.bench(
        "fast-high-error",
        ParameterizedBenchmark::new(
            "fast",
            move |b, p| b.iter(|| unsafe { fast.analyse(DATA.get(p).iter().cloned()) }),
            TRACKS.iter(),
        )
        .with_function("medium", move |b, p| {
            b.iter(|| unsafe { medium.analyse(DATA.get(p).iter().cloned()) })
        })
        .with_function("slow", move |b, p| {
            b.iter(|| unsafe { slow.analyse(DATA.get(p).iter().cloned()) })
        }),
    );
}

criterion_group!(benches, bench_tracks, bench_estimators);
criterion_main!(benches);
