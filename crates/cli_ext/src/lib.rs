use std::str::FromStr;

use clap::{ArgEnum, ArgMatches, Error, PossibleValue};

#[derive(Clone, ArgEnum)]
pub enum FileFormat {
    Json,
    Binary,
}

impl FileFormat {
    pub fn possible_values<'a>() -> impl Iterator<Item = PossibleValue<'a>> {
        FileFormat::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }

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
