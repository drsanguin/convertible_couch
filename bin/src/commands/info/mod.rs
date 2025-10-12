use clap::{builder::PossibleValue, ValueEnum};

/// Models the choice of which device(s) one wishes to obtain information from
#[derive(Clone, Debug, PartialEq)]
pub enum Device {
    /// Get informations about displays and speakers
    DisplaysAndSpeakers,
    /// Get informations about displays only
    DisplaysOnly,
    /// Get informations about speakers only
    SpeakersOnly,
}

impl ValueEnum for Device {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::DisplaysAndSpeakers,
            Self::DisplaysOnly,
            Self::SpeakersOnly,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let canonical_argument_value = match self {
            Self::DisplaysAndSpeakers => PossibleValue::new("displays-and-speakers"),
            Self::DisplaysOnly => PossibleValue::new("displays-only"),
            Self::SpeakersOnly => PossibleValue::new("speakers-only"),
        };

        Some(canonical_argument_value)
    }
}

#[cfg(test)]
mod tests {
    use clap::{builder::PossibleValue, ValueEnum};
    use test_case::test_case;

    use crate::commands::info::Device;

    #[test]
    fn it_should_provide_all_possible_argument_values() {
        // Act
        let value_variants = Device::value_variants();

        // Assert
        assert_eq!(
            value_variants,
            [
                Device::DisplaysAndSpeakers,
                Device::DisplaysOnly,
                Device::SpeakersOnly
            ]
        );
    }

    #[test_case(Device::DisplaysAndSpeakers => Some(PossibleValue::new("displays-and-speakers")); "when device is displays-and-speakers")]
    #[test_case(Device::DisplaysOnly => Some(PossibleValue::new("displays-only")); "when device is displays-only")]
    #[test_case(Device::SpeakersOnly => Some(PossibleValue::new("speakers-only")); "when device is speakers-only")]
    fn it_should_provide_the_canonical_argument_value(log_level: Device) -> Option<PossibleValue> {
        // Act
        log_level.to_possible_value()
    }
}
