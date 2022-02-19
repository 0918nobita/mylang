use clap::Parser;

pub trait MatchesFromParser<P>
where
    P: Parser,
{
    fn parse(&self) -> P;
}
