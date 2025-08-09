#![cfg(target_os = "windows")]

use convertible_couch::{
    application::ApplicationResult,
    testing::arrangements::{bootstrap_application, ArgumentsBuilder},
};
use convertible_couch_lib::{
    displays_settings::DisplaysSettingsResult, func, testing::fuzzing::Fuzzer,
};
use test_case::test_case;
use windows::Win32::Graphics::Gdi::{
    DISP_CHANGE, DISP_CHANGE_BADDUALVIEW, DISP_CHANGE_BADFLAGS, DISP_CHANGE_BADMODE,
    DISP_CHANGE_BADPARAM, DISP_CHANGE_FAILED, DISP_CHANGE_NOTUPDATED, DISP_CHANGE_RESTART,
};

#[test_case(DISP_CHANGE_BADDUALVIEW => Err(String::from("The settings change was unsuccessful because the system is DualView capable.")); "when the error is BADDUALVIEW")]
#[test_case(DISP_CHANGE_BADFLAGS => Err(String::from("An invalid set of flags was passed in.")); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(DISP_CHANGE_BADMODE => Err(String::from("The graphics mode is not supported.")); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(DISP_CHANGE_BADPARAM => Err(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(DISP_CHANGE_FAILED => Err(String::from("The display driver failed the specified graphics mode.")); "when the error is DISP_CHANGE_FAILED")]
#[test_case(DISP_CHANGE_NOTUPDATED => Err(String::from("Unable to write settings to the registry.")); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_when_committing_changes(
    disp_change: DISP_CHANGE,
) -> Result<ApplicationResult, String> {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_committing_the_display_changes_fails_with(disp_change)
        .build_displays()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .displays_only(
            primary_display_name.as_str(),
            secondary_display_name.as_str(),
        )
        .build();

    // Act
    application.execute(&args)
}

#[test_case(DISP_CHANGE_BADDUALVIEW => Err(String::from("The settings change was unsuccessful because the system is DualView capable.")); "when the error is BADDUALVIEW")]
#[test_case(DISP_CHANGE_BADFLAGS => Err(String::from("An invalid set of flags was passed in.")); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(DISP_CHANGE_BADMODE => Err(String::from("The graphics mode is not supported.")); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(DISP_CHANGE_BADPARAM => Err(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(DISP_CHANGE_FAILED => Err(String::from("The display driver failed the specified graphics mode.")); "when the error is DISP_CHANGE_FAILED")]
#[test_case(DISP_CHANGE_NOTUPDATED => Err(String::from("Unable to write settings to the registry.")); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_for_some_displays(
    disp_change: DISP_CHANGE,
) -> Result<ApplicationResult, String> {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_changing_the_display_settings_fails_for_some_displays(disp_change)
        .build_displays()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .displays_only(
            primary_display_name.as_str(),
            secondary_display_name.as_str(),
        )
        .build();

    // Act
    application.execute(&args)
}

#[test]
fn it_should_change_the_primary_display_of_computer_and_ask_for_reboot_when_required_after_committing_display_changes(
) {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_committing_the_display_changes_fails_with(DISP_CHANGE_RESTART)
        .build_displays()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .displays_only(
            primary_display_name.as_str(),
            secondary_display_name.as_str(),
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::DisplaysOnly {
            displays_result: DisplaysSettingsResult {
                new_primary_display: secondary_display_name,
                reboot_required: true,
            }
        })
    );
}

#[test]
fn it_should_change_the_primary_display_of_computer_and_ask_for_reboot_when_required_after_changing_display_for_some_displays(
) {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_changing_the_display_settings_fails_for_some_displays(DISP_CHANGE_RESTART)
        .build_displays()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::DisplaysOnly {
            displays_result: DisplaysSettingsResult {
                new_primary_display: secondary_display_name,
                reboot_required: true,
            }
        })
    );
}
