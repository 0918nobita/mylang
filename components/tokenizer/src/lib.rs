mod iter;

use std::io::BufRead;

use ast::{pos::Pos, range::Range};
use thiserror::Error;
use token::{KeywordKind, Token};
use utf8_chars::BufReadCharsExt;

use iter::WithPosExt;

#[derive(Clone)]
enum State {
    Initial,
    I32(Pos, String),
    Str {
        start: Pos,
        escape: bool,
        acc: String,
    },
    Keyword(Pos, String),
}

#[derive(Debug, Error)]
pub enum TokenizeError {
    #[error("({0}) Forbidden character: '{1}'")]
    ForbiddenChar(Pos, char),

    #[error("({0}) Invalid escape sequence: '{1}'")]
    InvalidEscapeSequence(Pos, char),

    #[error("({0}) Invalid keyword: \"{1}\"")]
    InvalidKeyword(Range, String),

    #[error("({0}) Missing a closing quote for string literal")]
    MissingClosingQuoteForStr(Pos),
}

type TokenizeResult = Result<Token, TokenizeError>;

fn tokenize_i32(start: &Pos, pos: &Pos, acc: &str) -> Token {
    let i = acc.parse::<i32>().unwrap();
    Token::I32(Range::new(start.clone(), pos.clone()), i)
}

fn try_tokenize_keyword(start: &Pos, pos: &Pos, acc: &str) -> TokenizeResult {
    if let Some(keyword_kind) = KeywordKind::parse(acc) {
        Ok(Token::Keyword(
            Range::new(start.clone(), pos.clone()),
            keyword_kind,
        ))
    } else {
        Err(TokenizeError::InvalidKeyword(
            Range::new(start.clone(), pos.clone()),
            acc.to_string(),
        ))
    }
}

pub fn tokenize<T: BufRead>(src: &mut T) -> impl Iterator<Item = TokenizeResult> + '_ {
    src.chars()
        .map(|r| r.unwrap())
        .with_pos()
        .scan(State::Initial, |state, (pos, c)| match (state.clone(), c) {
            (State::Initial, '\n') => Some(vec![Ok(Token::Newline(pos))]),
            (State::Initial, '"') => {
                *state = State::Str {
                    start: pos,
                    escape: false,
                    acc: String::new(),
                };
                Some(vec![])
            }
            (State::Initial, '+') => Some(vec![Ok(Token::AddOp(pos))]),
            (State::Initial, c) if c.is_ascii_whitespace() => Some(vec![]),
            (State::Initial, c) if c.is_ascii_digit() => {
                *state = State::I32(pos, c.to_string());
                Some(vec![])
            }
            (State::Initial, c) if c.is_ascii_alphabetic() => {
                *state = State::Keyword(pos, c.to_string());
                Some(vec![])
            }
            (State::Initial, _) => {
                *state = State::Initial;
                Some(vec![Err(TokenizeError::ForbiddenChar(pos, c))])
            }

            (State::I32(_, _), '_') => Some(vec![]),
            (State::I32(start, acc), '\n') => {
                *state = State::Initial;
                Some(vec![
                    Ok(tokenize_i32(&start, &pos, &acc)),
                    Ok(Token::Newline(pos.clone())),
                ])
            }
            (State::I32(start, acc), '"') => {
                *state = State::Str {
                    start: pos.clone(),
                    escape: false,
                    acc: String::new(),
                };
                Some(vec![Ok(tokenize_i32(&start, &pos, &acc))])
            }
            (State::I32(start, acc), '+') => Some(vec![
                Ok(tokenize_i32(&start, &pos, &acc)),
                Ok(Token::AddOp(pos.clone())),
            ]),
            (State::I32(start, acc), c) if c.is_ascii_digit() => {
                *state = State::I32(start, format!("{acc}{c}"));
                Some(vec![])
            }
            (State::I32(start, acc), c) if c.is_ascii_alphabetic() => {
                *state = State::Keyword(pos.clone(), c.to_string());
                Some(vec![Ok(tokenize_i32(&start, &pos, &acc))])
            }
            (State::I32(start, acc), c) if c.is_ascii_whitespace() => {
                *state = State::Initial;
                Some(vec![Ok(tokenize_i32(&start, &pos, &acc))])
            }
            (State::I32(_, _), _) => {
                *state = State::Initial;
                Some(vec![Err(TokenizeError::ForbiddenChar(pos, c))])
            }

            (State::Str { .. }, '\n') => {
                *state = State::Initial;
                Some(vec![Err(TokenizeError::MissingClosingQuoteForStr(pos))])
            }
            (
                State::Str {
                    start,
                    escape: false,
                    acc,
                },
                '\\',
            ) => {
                *state = State::Str {
                    start,
                    escape: true,
                    acc,
                };
                Some(vec![])
            }
            (
                State::Str {
                    start,
                    escape: true,
                    acc,
                },
                '"',
            ) => {
                *state = State::Str {
                    start,
                    escape: false,
                    acc: format!("{acc}\""),
                };
                Some(vec![])
            }
            (
                State::Str {
                    start,
                    escape: true,
                    acc,
                },
                'n',
            ) => {
                *state = State::Str {
                    start,
                    escape: false,
                    acc: format!("{acc}\n"),
                };
                Some(vec![])
            }
            (State::Str { escape: true, .. }, c) => {
                *state = State::Initial;
                Some(vec![Err(TokenizeError::InvalidEscapeSequence(pos, c))])
            }
            (
                State::Str {
                    start,
                    escape: false,
                    acc,
                },
                '"',
            ) => {
                *state = State::Initial;
                Some(vec![Ok(Token::Str(Range::new(start, pos), acc))])
            }
            (
                State::Str {
                    start,
                    escape: false,
                    acc,
                },
                c,
            ) => {
                *state = State::Str {
                    start,
                    escape: false,
                    acc: format!("{acc}{c}"),
                };
                Some(vec![])
            }

            (State::Keyword(start, acc), '\n') => {
                *state = State::Initial;
                Some(vec![
                    try_tokenize_keyword(&start, &pos, &acc),
                    Ok(Token::Newline(pos.clone())),
                ])
            }
            (State::Keyword(start, acc), '"') => {
                *state = State::Str {
                    start: pos.clone(),
                    escape: false,
                    acc: String::new(),
                };
                Some(vec![try_tokenize_keyword(&start, &pos, &acc)])
            }
            (State::Keyword(start, acc), '+') => {
                *state = State::Initial;
                Some(vec![
                    try_tokenize_keyword(&start, &pos, &acc),
                    Ok(Token::AddOp(pos.clone())),
                ])
            }
            (State::Keyword(start, acc), c) if c.is_ascii_whitespace() => {
                *state = State::Initial;
                Some(vec![try_tokenize_keyword(&start, &pos, &acc)])
            }
            (State::Keyword(start, acc), c) if c.is_ascii() => {
                *state = State::Keyword(start, format!("{acc}{c}"));
                Some(vec![])
            }
            (State::Keyword(_, _), _) => {
                *state = State::Initial;
                Some(vec![Err(TokenizeError::ForbiddenChar(pos, c))])
            }
        })
        .flatten()
}
