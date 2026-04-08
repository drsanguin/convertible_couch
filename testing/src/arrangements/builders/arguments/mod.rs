use crate::arrangements::builders::arguments::{
    change::ChangeCommandBuilder, displays::DisplaysCommandBuilder, info::InfoCommandBuilder,
    speakers::SpeakersCommandBuilder,
};

pub mod change;
pub mod displays;
pub mod info;
pub mod speakers;

pub struct ArgumentsBuilder;

impl ArgumentsBuilder {
    pub fn change() -> ChangeCommandBuilder {
        ChangeCommandBuilder::default()
    }

    pub fn info() -> InfoCommandBuilder {
        InfoCommandBuilder::default()
    }

    pub fn displays() -> DisplaysCommandBuilder {
        DisplaysCommandBuilder
    }

    pub fn speakers() -> SpeakersCommandBuilder {
        SpeakersCommandBuilder
    }
}
