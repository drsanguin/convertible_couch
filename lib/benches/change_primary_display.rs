use convertible_couch_lib::{
    display_settings::{CurrentDisplaySettings, DisplaySettings},
    func,
    testing::fuzzing::Fuzzer,
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

fn change_primary_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_primary_display");

    for n_display in [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144] {
        group.throughput(Throughput::Elements(u64::try_from(n_display).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(n_display),
            &n_display,
            |bencher, n_display| {
                bencher.iter_batched(
                    || {
                        let mut fuzzer = Fuzzer::new(func!(), false);

                        let computer = fuzzer
                            .generate_computer()
                            .with_displays()
                            .of_which_there_are(*n_display)
                            .build_displays()
                            .build_computer();

                        let display_settings =
                            CurrentDisplaySettings::new(computer.display_settings_api);

                        (
                            display_settings,
                            computer.primary_display,
                            computer.secondary_display,
                        )
                    },
                    |(mut display_settings, primary_display, secondary_display)| {
                        display_settings
                            .change_primary_display(&primary_display, &secondary_display)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, change_primary_display);
criterion_main!(benches);
