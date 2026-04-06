#![cfg(target_os = "windows")]

use convertible_couch_lib::func;
use convertible_couch_testing::{
    arrangements::{
        builders::{ApplicationBuilder, ArgumentsBuilder, DisplaysCommand},
        fuzzing::{ComputerBuilder, Fuzzer, displays::Function},
    },
    assertions::assert_that_result_is_a_win32_error,
};
use test_case::test_matrix;
use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;

#[test_matrix(
    [
        DisplaysCommand::ChangeDisplaysAndSpeakers,
        DisplaysCommand::ChangeDisplays,
        DisplaysCommand::InfoDisplaysAndSpeakers,
        DisplaysCommand::InfoDisplays
    ], [
        Function::GetDisplayConfigBufferSizes,
        Function::QueryDisplayConfig,
        Function::DisplayConfigGetDeviceInfo
    ]; "when"
)]
fn report_any_display_error(displays_command: DisplaysCommand, function: Function) {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();
    let forbidden_errors = match function {
        Function::QueryDisplayConfig => vec![ERROR_INSUFFICIENT_BUFFER], // ERROR_INSUFFICIENT_BUFFER would make QueryDisplayConfig to go into a infinite loop
        _ => vec![],
    };
    let win_32_error = fuzzer.generate_win_32_error(&forbidden_errors);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_function_fails_with(function, win_32_error)
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::display_command(
        displays_command,
        &primary_display_name,
        &secondary_display_name,
        &default_speaker_name,
        &alternative_speaker_name,
    );

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, win_32_error);
}
