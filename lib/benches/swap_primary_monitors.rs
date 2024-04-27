use convertible_couch_common_tests::new_fuzzer_no_seed_print;
use convertible_couch_lib::display_settings::DisplaySettings;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

fn swap_primary_monitors(c: &mut Criterion) {
    let mut group = c.benchmark_group("swap_primary_monitors");

    for n_monitor in [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144] {
        group.throughput(Throughput::Elements(u64::try_from(n_monitor).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(n_monitor),
            &n_monitor,
            |bencher, n_monitor| {
                bencher.iter_batched(
                    || {
                        let mut fuzzer = new_fuzzer_no_seed_print!();

                        let computer = fuzzer
                            .generate_computer()
                            .with_monitors()
                            .of_which_there_are(*n_monitor)
                            .build_monitors()
                            .build_computer();

                        let display_settings = DisplaySettings::new(computer.win32);

                        (
                            display_settings,
                            computer.primary_monitor,
                            computer.secondary_monitor,
                        )
                    },
                    |(mut display_settings, primary_monitor, secondary_monitor)| {
                        display_settings.swap_primary_monitors(&primary_monitor, &secondary_monitor)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, swap_primary_monitors);
criterion_main!(benches);
