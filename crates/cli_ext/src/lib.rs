use std::str::FromStr;

use clap::{ArgEnum, ArgMatches, Error, PossibleValue};
use once_cell::sync::Lazy;

#[derive(Clone, ArgEnum)]
pub enum FileFormat {
    Json,
    Binary,
}

impl FileFormat {
    pub fn value_of(matches: &ArgMatches, name: &str) -> Result<Self, Error> {
        matches.value_of_t::<Self>(name)
    }
}

impl FromStr for FileFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(FileFormat::Json),
            "binary" => Ok(FileFormat::Binary),
            _ => Err(format!("Invalid input format: {s}")),
        }
    }
}

pub static FILE_FORMAT_POSSIBLE_VALUES: Lazy<Vec<PossibleValue<'static>>> = Lazy::new(|| {
    FileFormat::value_variants()
        .iter()
        .filter_map(ArgEnum::to_possible_value)
        .collect()
});
