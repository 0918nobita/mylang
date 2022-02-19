use std::marker::PhantomData;

use clap::{Command, Parser};

pub struct CommandFromParser<'a, P>
where
    P: Parser,
{
    pub(crate) cmd: Command<'a>,
    parser: PhantomData<P>,
}

impl<'a, P> CommandFromParser<'a, P>
where
    P: Parser,
{
    pub fn new(cmd: Command<'a>) -> Self {
        Self {
            cmd,
            parser: PhantomData,
        }
    }
}

pub trait CommandFromParserExt<'a, P>
where
    P: Parser,
{
    fn to_command() -> CommandFromParser<'a, P>;
}

impl<'a, P> CommandFromParserExt<'a, P> for P
where
    P: Parser,
{
    fn to_command() -> CommandFromParser<'a, P> {
        CommandFromParser::new(P::command())
    }
}
