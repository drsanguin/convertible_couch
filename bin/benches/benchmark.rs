use std::fmt::Display;

use convertible_couch::{run_app, Arguments, AudioOpts, Commands, SharedOpts, VideoOpts};
use convertible_couch_lib::{
    display_settings::{CurrentDisplaySettings, DisplaySettings},
    func,
    log::LogLevel,
    sound_settings::{CurrentSoundSettings, SoundSettings},
    testing::fuzzing::Fuzzer,
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

const COUNTS: [usize; 10] = [2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

struct BenchParam {
    pub display_count: usize,
    pub audio_output_count: usize,
}

impl Display for BenchParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "display_count: {}, audio_output_count: {}",
            self.display_count, self.audio_output_count
        )
    }
}

fn swap_video_and_audio(c: &mut Criterion) {
    let mut group = c.benchmark_group("swap_video_and_audio");

    for display_count in COUNTS {
        for audio_output_count in COUNTS {
            let bench_parameter = BenchParam {
                display_count,
                audio_output_count,
            };

            group.throughput(Throughput::Elements(
                u64::try_from(display_count + audio_output_count).unwrap(),
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
                            let (default_audio_output_name, alternative_audio_output_name) =
                                fuzzer.generate_two_audio_output_devices_names();

                            let computer = fuzzer
                                .generate_computer()
                                .with_displays()
                                .of_which_there_are(bench_parameter.display_count)
                                .whose_primary_is_named(primary_display_name.clone())
                                .with_a_secondary_named(secondary_display_name.clone())
                                .build_displays()
                                .with_audio_output_devices()
                                .of_which_there_are(bench_parameter.audio_output_count)
                                .whose_default_one_is_named(default_audio_output_name.clone())
                                .with_an_alternative_one_named(
                                    alternative_audio_output_name.clone(),
                                )
                                .build_audio_output_devices()
                                .build_computer();

                            let display_settings =
                                CurrentDisplaySettings::new(computer.display_settings_api);
                            let sound_settings =
                                CurrentSoundSettings::new(computer.audio_settings_api);

                            let args = Arguments {
                                command: Commands::VideoAndAudio {
                                    video: VideoOpts {
                                        desktop_display_name: primary_display_name,
                                        couch_display_name: secondary_display_name,
                                    },
                                    audio: AudioOpts {
                                        desktop_speaker_name: default_audio_output_name,
                                        couch_speaker_name: alternative_audio_output_name,
                                    },
                                    shared: SharedOpts {
                                        log_level: LogLevel::Off,
                                    },
                                },
                            };

                            (args, display_settings, sound_settings)
                        },
                        |(args, mut display_settings, mut sound_settings)| {
                            run_app(&args, &mut display_settings, &mut sound_settings)
                        },
                        BatchSize::SmallInput,
                    );
                },
            );
        }
    }
    group.finish();
}

fn swap_video_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("swap_video_only");

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

                        let display_settings =
                            CurrentDisplaySettings::new(computer.display_settings_api);
                        let sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

                        let args = Arguments {
                            command: Commands::VideoOnly {
                                video: VideoOpts {
                                    desktop_display_name: primary_display_name,
                                    couch_display_name: secondary_display_name,
                                },
                                shared: SharedOpts {
                                    log_level: LogLevel::Off,
                                },
                            },
                        };

                        (args, display_settings, sound_settings)
                    },
                    |(args, mut display_settings, mut sound_settings)| {
                        run_app(&args, &mut display_settings, &mut sound_settings)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn swap_audio_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("swap_audio_only");

    for audio_output_count in COUNTS {
        group.throughput(Throughput::Elements(
            u64::try_from(audio_output_count).unwrap(),
        ));
        group.bench_with_input(
            BenchmarkId::from_parameter(&audio_output_count),
            &audio_output_count,
            |bencher, audio_output_count| {
                bencher.iter_batched(
                    || {
                        let mut fuzzer = Fuzzer::new(func!(), false);

                        let (default_audio_output_name, alternative_audio_output_name) =
                            fuzzer.generate_two_audio_output_devices_names();

                        let computer = fuzzer
                            .generate_computer()
                            .with_audio_output_devices()
                            .of_which_there_are(*audio_output_count)
                            .whose_default_one_is_named(default_audio_output_name.clone())
                            .with_an_alternative_one_named(alternative_audio_output_name.clone())
                            .build_audio_output_devices()
                            .build_computer();

                        let display_settings =
                            CurrentDisplaySettings::new(computer.display_settings_api);
                        let sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

                        let args = Arguments {
                            command: Commands::AudioOnly {
                                audio: AudioOpts {
                                    desktop_speaker_name: default_audio_output_name,
                                    couch_speaker_name: alternative_audio_output_name,
                                },
                                shared: SharedOpts {
                                    log_level: LogLevel::Off,
                                },
                            },
                        };

                        (args, display_settings, sound_settings)
                    },
                    |(args, mut display_settings, mut sound_settings)| {
                        run_app(&args, &mut display_settings, &mut sound_settings)
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
    swap_video_and_audio,
    swap_video_only,
    swap_audio_only
);
criterion_main!(benches);
