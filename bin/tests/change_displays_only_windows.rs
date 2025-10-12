#![cfg(target_os = "windows")]

use convertible_couch::{
    application::{ApplicationChangeResult, ApplicationResult},
    testing::arrangements::{bootstrap_application, ArgumentsBuilder},
};
use convertible_couch_lib::{
    displays_settings::DisplaysSettingsResult,
    func,
    testing::fuzzing::{ComputerBuilder, Fuzzer},
    ApplicationError,
};
use test_case::test_case;
use windows::Win32::{
    Foundation::ERROR_INVALID_PARAMETER,
    Graphics::Gdi::{
        DISP_CHANGE, DISP_CHANGE_BADDUALVIEW, DISP_CHANGE_BADFLAGS, DISP_CHANGE_BADMODE,
        DISP_CHANGE_BADPARAM, DISP_CHANGE_FAILED, DISP_CHANGE_NOTUPDATED, DISP_CHANGE_RESTART,
    },
};

#[test_case(DISP_CHANGE_BADDUALVIEW => Err(ApplicationError::Custom(String::from("The settings change was unsuccessful because the system is DualView capable."))); "when the error is BADDUALVIEW")]
#[test_case(DISP_CHANGE_BADFLAGS => Err(ApplicationError::Custom(String::from("An invalid set of flags was passed in."))); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(DISP_CHANGE_BADMODE => Err(ApplicationError::Custom(String::from("The graphics mode is not supported."))); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(DISP_CHANGE_BADPARAM => Err(ApplicationError::Custom(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags."))); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(DISP_CHANGE_FAILED => Err(ApplicationError::Custom(String::from("The display driver failed the specified graphics mode."))); "when the error is DISP_CHANGE_FAILED")]
#[test_case(DISP_CHANGE_NOTUPDATED => Err(ApplicationError::Custom(String::from("Unable to write settings to the registry."))); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_committed_displays_settings_changes_errors(
    disp_change: DISP_CHANGE,
) -> Result<ApplicationResult, ApplicationError> {
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
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    application.execute(&args)
}

#[test_case(DISP_CHANGE_BADDUALVIEW => Err(ApplicationError::Custom(String::from("The settings change was unsuccessful because the system is DualView capable."))); "when the error is BADDUALVIEW")]
#[test_case(DISP_CHANGE_BADFLAGS => Err(ApplicationError::Custom(String::from("An invalid set of flags was passed in."))); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(DISP_CHANGE_BADMODE => Err(ApplicationError::Custom(String::from("The graphics mode is not supported."))); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(DISP_CHANGE_BADPARAM => Err(ApplicationError::Custom(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags."))); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(DISP_CHANGE_FAILED => Err(ApplicationError::Custom(String::from("The display driver failed the specified graphics mode."))); "when the error is DISP_CHANGE_FAILED")]
#[test_case(DISP_CHANGE_NOTUPDATED => Err(ApplicationError::Custom(String::from("Unable to write settings to the registry."))); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_displays_settings_changes_errors(
    disp_change: DISP_CHANGE,
) -> Result<ApplicationResult, ApplicationError> {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_changing_the_display_settings_fails_with(disp_change)
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    application.execute(&args)
}

#[test]
fn it_should_ask_for_reboot_when_committing_displays_settings_requires_it() {
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
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Change(
            ApplicationChangeResult::DisplaysOnly {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: secondary_display_name,
                    reboot_required: true,
                }
            }
        ))
    );
}

#[test]
fn it_should_ask_for_reboot_when_changing_displays_settings_requires_it() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_changing_the_display_settings_fails_with(DISP_CHANGE_RESTART)
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Change(
            ApplicationChangeResult::DisplaysOnly {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: secondary_display_name,
                    reboot_required: true,
                }
            }
        ))
    );
}

#[test]
fn it_should_report_get_display_config_buffer_sizes_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_getting_display_config_buffer_sizes_fails_with(ERROR_INVALID_PARAMETER)
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(format!(
            "Failed to retrieve the size of the buffers that are required to call the QueryDisplayConfig function: {}",
            ERROR_INVALID_PARAMETER.0)))
    );
}

#[test]
fn it_should_report_query_display_config_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_querying_display_config_fails_with(ERROR_INVALID_PARAMETER)
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(format!(
            "Failed to retrieve information about all possible display paths for all display devices, or views, in the current setting: {}",
            ERROR_INVALID_PARAMETER.0)))
    );
}

#[test]
fn it_should_handle_the_case_of_a_display_being_not_possible_to_enum_display_settings_on() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name, secondary_display_name_2) =
        fuzzer.generate_three_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(3)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .with_a_secondary_for_which_it_is_not_possible_to_enum_display_settings_on(
            secondary_display_name_2,
        )
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(String::from(
            "The display driver failed the specified graphics mode."
        )))
    );
}
