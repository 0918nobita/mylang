use std::marker::PhantomData;

use clap::{Command, Parser};

pub struct CommandFromParser<P>
where
    P: Parser,
{
    pub(crate) cmd: Command,
    parser: PhantomData<P>,
}

impl<P> CommandFromParser<P>
where
    P: Parser,
{
    pub fn new(cmd: Command) -> Self {
        Self {
            cmd,
            parser: PhantomData,
        }
    }
}

pub trait CommandFromParserExt<P>
where
    P: Parser,
{
    fn to_command() -> CommandFromParser<P>;
}

impl<P> CommandFromParserExt<P> for P
where
    P: Parser,
{
    fn to_command() -> CommandFromParser<P> {
        CommandFromParser::new(P::command())
    }
}
