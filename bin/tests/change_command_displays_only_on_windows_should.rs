#![cfg(target_os = "windows")]

use convertible_couch_lib::func;
use convertible_couch_testing::{
    arrangements::{
        builders::{ApplicationBuilder, ArgumentsBuilder},
        fuzzing::{ComputerBuilder, Fuzzer},
    },
    assertions::assert_that_result_is_a_win32_error,
};
use windows::Win32::Foundation::ERROR_INVALID_PARAMETER;

#[test]
fn report_get_display_config_buffer_sizes_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_get_display_config_buffer_fails_with(ERROR_INVALID_PARAMETER)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}

#[test]
fn report_query_display_config_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_query_display_config_fails_with(vec![ERROR_INVALID_PARAMETER])
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}

#[test]
fn report_display_config_get_device_info_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_display_config_get_device_info_fails_with(ERROR_INVALID_PARAMETER)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}

#[test]
fn report_set_display_config_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_set_display_config_fails_with(ERROR_INVALID_PARAMETER)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}
