use std::fmt::Display;

use convertible_couch::{
    run_app, Arguments, Commands, DisplaysOptions, SharedOptions, SpeakersOptions,
};
use convertible_couch_lib::{
    displays_settings::{CurrentDisplaysSettings, DisplaysSettings},
    func,
    log::LogLevel,
    speakers_settings::{CurrentSpeakersSettings, SpeakersSettings},
    testing::fuzzing::Fuzzer,
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

const COUNTS: [usize; 10] = [2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

struct BenchParam {
    pub displays_count: usize,
    pub speakers_count: usize,
}

impl Display for BenchParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "displays_count: {}, speakers_count: {}",
            self.displays_count, self.speakers_count
        )
    }
}

fn change_primary_display_and_default_speaker(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_primary_display_and_default_speaker");

    for displays_count in COUNTS {
        for speakers_count in COUNTS {
            let bench_parameter = BenchParam {
                displays_count,
                speakers_count,
            };

            group.throughput(Throughput::Elements(
                u64::try_from(displays_count + speakers_count).unwrap(),
            ));
            group.bench_with_input(
                BenchmarkId::from_parameter(&bench_parameter),
                &bench_parameter,
                |bencher, bench_parameter| {
                    bencher.iter_batched(
                        || {
                            let mut fuzzer = Fuzzer::new(func!(), false);

                            let (primary_display_name, secondary_display_name) =
                                fuzzer.generate_two_display_names();
                            let (default_speaker_name, alternative_speaker_name) =
                                fuzzer.generate_two_speakers_names();

                            let computer = fuzzer
                                .generate_computer()
                                .with_displays()
                                .of_which_there_are(bench_parameter.displays_count)
                                .whose_primary_is_named(primary_display_name.clone())
                                .with_a_secondary_named(secondary_display_name.clone())
                                .build_displays()
                                .with_speakers()
                                .of_which_there_are(bench_parameter.speakers_count)
                                .whose_default_one_is_named(default_speaker_name.clone())
                                .with_an_alternative_one_named(alternative_speaker_name.clone())
                                .build_speakers()
                                .build_computer();

                            let displays_settings =
                                CurrentDisplaysSettings::new(computer.displays_settings_api);
                            let speakers_settings =
                                CurrentSpeakersSettings::new(computer.speakers_settings_api);

                            let args = Arguments {
                                command: Commands::DisplaysAndSpeakers {
                                    displays: DisplaysOptions {
                                        desktop_display_name: primary_display_name,
                                        couch_display_name: secondary_display_name,
                                    },
                                    speakers: SpeakersOptions {
                                        desktop_speaker_name: default_speaker_name,
                                        couch_speaker_name: alternative_speaker_name,
                                    },
                                    shared: SharedOptions {
                                        log_level: LogLevel::Off,
                                    },
                                },
                            };

                            (args, displays_settings, speakers_settings)
                        },
                        |(args, mut displays_settings, mut speakers_settings)| {
                            run_app(&args, &mut displays_settings, &mut speakers_settings)
                        },
                        BatchSize::SmallInput,
                    );
                },
            );
        }
    }
    group.finish();
}

fn change_primary_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_primary_display");

    for display_count in COUNTS {
        group.throughput(Throughput::Elements(u64::try_from(display_count).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(&display_count),
            &display_count,
            |bencher, display_count| {
                bencher.iter_batched(
                    || {
                        let mut fuzzer = Fuzzer::new(func!(), false);

                        let (primary_display_name, secondary_display_name) =
                            fuzzer.generate_two_display_names();

                        let computer = fuzzer
                            .generate_computer()
                            .with_displays()
                            .of_which_there_are(*display_count)
                            .whose_primary_is_named(primary_display_name.clone())
                            .with_a_secondary_named(secondary_display_name.clone())
                            .build_displays()
                            .build_computer();

                        let displays_settings =
                            CurrentDisplaysSettings::new(computer.displays_settings_api);
                        let speakers_settings =
                            CurrentSpeakersSettings::new(computer.speakers_settings_api);

                        let args = Arguments {
                            command: Commands::DisplaysOnly {
                                displays: DisplaysOptions {
                                    desktop_display_name: primary_display_name,
                                    couch_display_name: secondary_display_name,
                                },
                                shared: SharedOptions {
                                    log_level: LogLevel::Off,
                                },
                            },
                        };

                        (args, displays_settings, speakers_settings)
                    },
                    |(args, mut displays_settings, mut speakers_settings)| {
                        run_app(&args, &mut displays_settings, &mut speakers_settings)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn change_default_speaker(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_default_speaker");

    for speakers_count in COUNTS {
        group.throughput(Throughput::Elements(u64::try_from(speakers_count).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(&speakers_count),
            &speakers_count,
            |bencher, speakers_count| {
                bencher.iter_batched(
                    || {
                        let mut fuzzer = Fuzzer::new(func!(), false);

                        let (default_speaker_name, alternative_speaker_name) =
                            fuzzer.generate_two_speakers_names();

                        let computer = fuzzer
                            .generate_computer()
                            .with_speakers()
                            .of_which_there_are(*speakers_count)
                            .whose_default_one_is_named(default_speaker_name.clone())
                            .with_an_alternative_one_named(alternative_speaker_name.clone())
                            .build_speakers()
                            .build_computer();

                        let displays_settings =
                            CurrentDisplaysSettings::new(computer.displays_settings_api);
                        let speakers_settings =
                            CurrentSpeakersSettings::new(computer.speakers_settings_api);

                        let args = Arguments {
                            command: Commands::SpeakersOnly {
                                speakers: SpeakersOptions {
                                    desktop_speaker_name: default_speaker_name,
                                    couch_speaker_name: alternative_speaker_name,
                                },
                                shared: SharedOptions {
                                    log_level: LogLevel::Off,
                                },
                            },
                        };

                        (args, displays_settings, speakers_settings)
                    },
                    |(args, mut displays_settings, mut speakers_settings)| {
                        run_app(&args, &mut displays_settings, &mut speakers_settings)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    change_primary_display_and_default_speaker,
    change_primary_display,
    change_default_speaker
);
criterion_main!(benches);
