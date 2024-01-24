use convertible_couch_lib::display_settings::DisplaySettings;
use convertible_couch_tests_common::new_fuzzer_no_seed_print;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

fn swap_primary_monitors(c: &mut Criterion) {
    let mut group = c.benchmark_group("swap_primary_monitors");

    for n_monitor in [
        3, 6, 8, 11, 13, 14, 15, 17, 20, 25, 27, 30, 31, 34, 46, 55, 61, 66, 72, 88, 97, 98, 122,
        162,
    ] {
        group.throughput(Throughput::Elements(u64::try_from(n_monitor).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(n_monitor),
            &n_monitor,
            |b, n_monitor| {
                b.iter_batched(
                    || {
                        let mut fuzzer = new_fuzzer_no_seed_print!();

                        let computer = fuzzer
                            .generate_a_computer()
                            .with_n_monitors(*n_monitor)
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
