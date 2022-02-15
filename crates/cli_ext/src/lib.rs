use std::str::FromStr;

use clap::{Arg, ArgEnum, ArgMatches, Error};

#[derive(Clone, ArgEnum)]
pub enum InputFormat {
    Json,
    Binary,
}

impl FromStr for InputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(InputFormat::Json),
            "binary" => Ok(InputFormat::Binary),
            _ => Err(format!("Invalid input format: {s}")),
        }
    }
}

pub fn input_format_arg<'a>() -> Arg<'a> {
    Arg::new("input_format")
        .long("input_format")
        .takes_value(true)
        .possible_values(
            InputFormat::value_variants()
                .iter()
                .filter_map(ArgEnum::to_possible_value),
        )
}

pub fn value_of_input_format(matches: &ArgMatches) -> Result<InputFormat, Error> {
    matches.value_of_t::<InputFormat>("input_format")
}
