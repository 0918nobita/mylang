mod iter;

use std::io::BufRead;

use ast::{pos::Pos, range::Range};
use thiserror::Error;
use token::{KeywordKind, Token};
use utf8_chars::BufReadCharsExt;

use iter::WithPosExt;

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

fn tokenize_i32(tokens: &mut Vec<TokenizeResult>, start: &Pos, pos: &Pos, acc: &str) {
    let i = acc.parse::<i32>().unwrap();
    tokens.push(Ok(Token::I32(Range::new(start.clone(), pos.clone()), i)));
}

fn try_tokenize_keyword(tokens: &mut Vec<TokenizeResult>, start: &Pos, pos: &Pos, acc: &str) {
    tokens.push(if let Some(keyword_kind) = KeywordKind::parse(acc) {
        Ok(Token::Keyword(
            Range::new(start.clone(), pos.clone()),
            keyword_kind,
        ))
    } else {
        Err(TokenizeError::InvalidKeyword(
            Range::new(start.clone(), pos.clone()),
            acc.to_string(),
        ))
    });
}

pub fn tokenize<T: BufRead>(src: &mut T) -> Vec<TokenizeResult> {
    let mut tokens = Vec::<Result<Token, TokenizeError>>::new();
    let mut state = State::Initial;

    for (pos, c) in src.chars().map(|r| r.unwrap()).with_pos() {
        match (&state, c) {
            (State::Initial, '\n') => tokens.push(Ok(Token::Newline(pos.clone()))),
            (State::Initial, '"') => {
                state = State::Str {
                    start: pos.clone(),
                    escape: false,
                    acc: String::new(),
                }
            }
            (State::Initial, '+') => tokens.push(Ok(Token::AddOp(pos.clone()))),
            (State::Initial, c) if c.is_ascii_whitespace() => (),
            (State::Initial, c) if c.is_ascii_digit() => state = State::I32(pos, c.to_string()),
            (State::Initial, c) if c.is_ascii_alphabetic() => {
                state = State::Keyword(pos, c.to_string())
            }
            (State::Initial, _) => {
                tokens.push(Err(TokenizeError::ForbiddenChar(pos, c)));
                state = State::Initial
            }

            (State::I32(_, _), '_') => (),
            (State::I32(start, acc), '\n') => {
                tokenize_i32(&mut tokens, start, &pos, acc);
                tokens.push(Ok(Token::Newline(pos.clone())));
                state = State::Initial
            }
            (State::I32(start, acc), '"') => {
                tokenize_i32(&mut tokens, start, &pos, acc);
                state = State::Str {
                    start: pos.clone(),
                    escape: false,
                    acc: String::new(),
                }
            }
            (State::I32(start, acc), '+') => {
                tokenize_i32(&mut tokens, start, &pos, acc);
                tokens.push(Ok(Token::AddOp(pos.clone())))
            }
            (State::I32(start, acc), c) if c.is_ascii_digit() => {
                state = State::I32(start.clone(), format!("{acc}{c}"))
            }
            (State::I32(start, acc), c) if c.is_ascii_alphabetic() => {
                tokenize_i32(&mut tokens, start, &pos, acc);
                state = State::Keyword(pos, c.to_string())
            }
            (State::I32(start, acc), c) if c.is_ascii_whitespace() => {
                tokenize_i32(&mut tokens, start, &pos, acc);
                state = State::Initial
            }
            (State::I32(_, _), _) => {
                tokens.push(Err(TokenizeError::ForbiddenChar(pos, c)));
                state = State::Initial
            }

            (State::Str { .. }, '\n') => {
                tokens.push(Err(TokenizeError::MissingClosingQuoteForStr(pos.clone())));
                state = State::Initial
            }
            (
                State::Str {
                    start,
                    escape: false,
                    acc,
                },
                '\\',
            ) => {
                state = State::Str {
                    start: start.clone(),
                    escape: true,
                    acc: acc.clone(),
                }
            }
            (
                State::Str {
                    start,
                    escape: true,
                    acc,
                },
                '"',
            ) => {
                state = State::Str {
                    start: start.clone(),
                    escape: false,
                    acc: format!("{acc}\""),
                }
            }
            (
                State::Str {
                    start,
                    escape: true,
                    acc,
                },
                'n',
            ) => {
                state = State::Str {
                    start: start.clone(),
                    escape: false,
                    acc: format!("{acc}\n"),
                }
            }
            (State::Str { escape: true, .. }, c) => {
                tokens.push(Err(TokenizeError::InvalidEscapeSequence(pos.clone(), c)));
                state = State::Initial
            }
            (
                State::Str {
                    start,
                    escape: false,
                    acc,
                },
                '"',
            ) => {
                tokens.push(Ok(Token::Str(
                    Range::new(start.clone(), pos.clone()),
                    acc.clone(),
                )));
                state = State::Initial
            }
            (
                State::Str {
                    start,
                    escape: false,
                    acc,
                },
                c,
            ) => {
                state = State::Str {
                    start: start.clone(),
                    escape: false,
                    acc: format!("{acc}{c}"),
                }
            }

            (State::Keyword(start, acc), '\n') => {
                try_tokenize_keyword(&mut tokens, start, &pos, acc);
                tokens.push(Ok(Token::Newline(pos.clone())));
                state = State::Initial
            }
            (State::Keyword(start, acc), '"') => {
                try_tokenize_keyword(&mut tokens, start, &pos, acc);
                state = State::Str {
                    start: pos.clone(),
                    escape: false,
                    acc: String::new(),
                }
            }
            (State::Keyword(start, acc), '+') => {
                try_tokenize_keyword(&mut tokens, start, &pos, acc);
                tokens.push(Ok(Token::AddOp(pos.clone())));
                state = State::Initial
            }
            (State::Keyword(start, acc), c) if c.is_ascii_whitespace() => {
                try_tokenize_keyword(&mut tokens, start, &pos, acc);
                state = State::Initial
            }
            (State::Keyword(start, acc), c) if c.is_ascii() => {
                state = State::Keyword(start.clone(), format!("{acc}{c}"))
            }
            (State::Keyword(_, _), _) => {
                tokens.push(Err(TokenizeError::ForbiddenChar(pos, c)));
                state = State::Initial
            }
        }
    }

    tokens
}
