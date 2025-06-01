use convertible_couch_common_tests::new_fuzzer_no_seed_print;
use convertible_couch_lib::display_settings::{self, DisplaySettings};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

fn swap_primary_displays(c: &mut Criterion) {
    let mut group = c.benchmark_group("swap_primary_displays");

    for n_display in [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144] {
        group.throughput(Throughput::Elements(u64::try_from(n_display).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(n_display),
            &n_display,
            |bencher, n_display| {
                bencher.iter_batched(
                    || {
                        let mut fuzzer = new_fuzzer_no_seed_print!();

                        let computer = fuzzer
                            .generate_computer()
                            .with_displays()
                            .of_which_there_are(*n_display)
                            .build_displays()
                            .build_computer();

                        let display_settings =
                            display_settings::Current::new(computer.display_settings_api);

                        (
                            display_settings,
                            computer.primary_display,
                            computer.secondary_display,
                        )
                    },
                    |(mut display_settings, primary_display, secondary_display)| {
                        display_settings.swap_primary_displays(&primary_display, &secondary_display)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, swap_primary_displays);
criterion_main!(benches);
