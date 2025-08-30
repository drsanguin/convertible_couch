use convertible_couch::testing::arrangements::{bootstrap_application, ArgumentsBuilder};
use convertible_couch_lib::{
    func,
    testing::fuzzing::{ComputerBuilder, Fuzzer},
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use std::fmt::{Display, Formatter, Result};

const COUNTS: [usize; 10] = [2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

struct BenchParam {
    pub displays_count: usize,
    pub speakers_count: usize,
}

impl Display for BenchParam {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "displays_count: {}, speakers_count: {}",
            self.displays_count, self.speakers_count
        )
    }
}

fn change_primary_display_and_default_speaker(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("change_primary_display_and_default_speaker");

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
                                .build_computer();

                            let application = bootstrap_application(computer);

                            let args = ArgumentsBuilder::new()
                                .displays_and_speakers(
                                    &primary_display_name,
                                    &secondary_display_name,
                                    &default_speaker_name,
                                    &alternative_speaker_name,
                                )
                                .build();

                            (application, args)
                        },
                        |(mut application, args)| application.execute(&args),
                        BatchSize::SmallInput,
                    );
                },
            );
        }
    }
    group.finish();
}

fn change_primary_display(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("change_primary_display");

    for display_count in COUNTS {
        group.throughput(Throughput::Elements(u64::try_from(display_count).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(display_count),
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
                            .build_computer();

                        let application = bootstrap_application(computer);

                        let args = ArgumentsBuilder::new()
                            .displays_only(&primary_display_name, &secondary_display_name)
                            .build();

                        (application, args)
                    },
                    |(mut application, args)| application.execute(&args),
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn change_default_speaker(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("change_default_speaker");

    for speakers_count in COUNTS {
        group.throughput(Throughput::Elements(u64::try_from(speakers_count).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(speakers_count),
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
                            .build_computer();

                        let application = bootstrap_application(computer);

                        let args = ArgumentsBuilder::new()
                            .speakers_only(&default_speaker_name, &alternative_speaker_name)
                            .build();

                        (application, args)
                    },
                    |(mut application, args)| application.execute(&args),
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
