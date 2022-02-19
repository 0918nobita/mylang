mod command;
mod matches;
mod with_output;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Stdin, Stdout, Write},
    str::FromStr,
};

use anyhow::anyhow;
use clap::{ArgEnum, ArgMatches, Error, PossibleValue};
use once_cell::sync::Lazy;

pub use command::{CommandFromParser, CommandFromParserExt};
pub use matches::MatchesFromParser;
pub use with_output::WithOutputExt;

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

pub fn reader_from_stdin_or_file<'a>(
    stdin: &'a Stdin,
    stdin_flag: bool,
    file_path: Option<&str>,
) -> anyhow::Result<Box<dyn BufRead + 'a>> {
    match (stdin_flag, file_path) {
        (true, Some(_)) => Err(anyhow!("Cannot specify both --stdin and [input]")),

        (true, None) => Ok(Box::new(stdin.lock())),

        (false, Some(path)) => Ok(Box::new(BufReader::new(File::open(path)?))),

        (false, None) => Err(anyhow!(
            "No input specified. You can specify either --stdin or [input]"
        )),
    }
}

pub fn read<T>(reader: Box<dyn BufRead + '_>, input_format: &FileFormat) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let value = match input_format {
        FileFormat::Json => serde_json::from_reader(reader)?,
        FileFormat::Binary => bincode::deserialize_from(reader)?,
    };
    Ok(value)
}

pub fn writer_to_stdout_or_file<'a>(
    stdout: &'a Stdout,
    stdout_flag: bool,
    file_path: Option<&str>,
) -> anyhow::Result<Box<dyn Write + 'a>> {
    match (stdout_flag, file_path) {
        (true, Some(_)) => Err(anyhow!("Cannot specify both --stdout and --output")),

        (true, None) => Ok(Box::new(stdout.lock())),

        (false, Some(path)) => Ok(Box::new(BufWriter::new(File::create(path)?))),

        (false, None) => Err(anyhow!(
            "No output specified. You can specify either --stdout or --output"
        )),
    }
}

pub fn write<T>(
    mut writer: Box<dyn Write + '_>,
    output_format: &FileFormat,
    value: &T,
) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    match output_format {
        FileFormat::Json => serde_json::to_writer_pretty(&mut writer, &value)?,
        FileFormat::Binary => bincode::serialize_into(&mut writer, &value)?,
    }
    Ok(())
}
