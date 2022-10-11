use std::{
    fs::File,
    io::{BufWriter, Stdout, Write},
    marker::PhantomData,
};

use clap::{Arg, ArgGroup, ArgMatches, Command, Parser};

use crate::{CommandFromParser, MatchesFromParser};

pub struct MatchesWithOutput<P>
where
    P: Parser,
{
    matches: ArgMatches,
    parser: PhantomData<P>,
}

impl<P> MatchesWithOutput<P>
where
    P: Parser,
{
    pub fn output<'a>(&self, stdout: &'a Stdout) -> anyhow::Result<Box<dyn Write + 'a>> {
        let stdout_flag = self.matches.contains_id("stdout");
        let file_path = self.matches.get_one::<String>("output");
        match (stdout_flag, file_path) {
            (true, Some(_)) => panic!("Cannot specify both --stdout and --output"),

            (true, None) => Ok(Box::new(stdout.lock())),

            (false, Some(path)) => Ok(Box::new(BufWriter::new(File::create(path)?))),

            (false, None) => {
                panic!("No output specified. You can specify either --stdout or --output")
            }
        }
    }
}

impl<P> MatchesFromParser<P> for MatchesWithOutput<P>
where
    P: Parser,
{
    fn parse(&self) -> P {
        P::from_arg_matches(&self.matches).unwrap()
    }
}

pub struct ParserWithOutput<P>
where
    P: Parser,
{
    cmd: Command,
    parser: PhantomData<P>,
}

impl<P> ParserWithOutput<P>
where
    P: Parser,
{
    pub fn new(cmd_from_parser: CommandFromParser<P>) -> Self {
        Self {
            cmd: cmd_from_parser
                .cmd
                .group(ArgGroup::new("output_group").required(true))
                .arg(Arg::new("stdout").long("stdout").group("output_group"))
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .group("output_group"),
                ),
            parser: PhantomData,
        }
    }

    pub fn get_matches(self) -> MatchesWithOutput<P> {
        MatchesWithOutput {
            matches: self.cmd.get_matches(),
            parser: PhantomData,
        }
    }
}

pub trait WithOutputExt<P>
where
    P: Parser,
{
    fn with_output(self) -> ParserWithOutput<P>;
}

impl<P> WithOutputExt<P> for CommandFromParser<P>
where
    P: Parser,
{
    fn with_output(self) -> ParserWithOutput<P> {
        ParserWithOutput::new(self)
    }
}
