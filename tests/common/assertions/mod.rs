use std::fmt::Display;

pub fn assert_that_expected_monitor_has_been_set(
    actual_primary_monitor: &str,
    expected_primary_monitor: &str,
) {
    assert_eq!(
        actual_primary_monitor, expected_primary_monitor,
        "Expected primary monitor to have been set to {} but it has been set to {}",
        expected_primary_monitor, actual_primary_monitor
    )
}

pub fn fail_because_primary_monitor_has_not_changed(expected_primary_monitor: &str) {
    assert!(
        false,
        "Expected primary monitor to have been set to {} but it has not been changed",
        expected_primary_monitor
    )
}

pub fn fail_because_an_error_occured<T: Display>(error: T, expected_primary_monitor: &str) {
    assert!(
        false,
        "Expected primary monitor to have been set to {} but if failed because of the error {}",
        expected_primary_monitor, error
    )
}
