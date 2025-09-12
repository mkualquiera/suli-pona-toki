use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{
    Natural,
    tokens::raw::{LITERALS, PREFIXES, ROOTS, SUFFIXES, SUFSUFFIXES},
};

mod raw;

struct FeaturedWord {
    root: &'static str,
    prefix: Option<&'static str>,
    suffix: Option<&'static str>,
    sufsuffix: Option<&'static str>,
}

fn make_map() -> HashMap<String, FeaturedWord> {
    let mut m = HashMap::new();

    let roots_iter = || ROOTS.iter().chain(SUFFIXES.iter()).chain(LITERALS.iter());

    let prefix_iter = || PREFIXES.iter().map(Some).chain([None]);
    let suffix_iter = || SUFFIXES.iter().map(Some).chain([None]);
    let sufsuffix_iter = || SUFSUFFIXES.iter().map(Some).chain([None]);

    for prefix in prefix_iter() {
        for root in roots_iter() {
            for suffix in suffix_iter() {
                if let Some(suffix) = suffix {
                    if suffix == root {
                        continue;
                    }
                }
                if suffix.is_some() {
                    for sufsuffix in sufsuffix_iter() {
                        let string_repr = format!(
                            "{}{}{}{}",
                            prefix.map_or("", |p| p),
                            root,
                            suffix.map_or("", |s| s),
                            sufsuffix.map_or("", |s| s)
                        );
                        m.insert(
                            string_repr,
                            FeaturedWord {
                                root,
                                prefix: prefix.map(|v| &**v),
                                suffix: suffix.map(|v| &**v),
                                sufsuffix: sufsuffix.map(|v| &**v),
                            },
                        );
                    }
                }
                let string_repr = format!(
                    "{}{}{}",
                    prefix.map_or("", |p| p),
                    root,
                    suffix.map_or("", |s| s),
                );
                m.insert(
                    string_repr,
                    FeaturedWord {
                        root,
                        prefix: prefix.map(|v| &**v),
                        suffix: suffix.map(|v| &**v),
                        sufsuffix: None,
                    },
                );
            }
        }
    }

    m
}

lazy_static! {
    static ref WORD_LOOKUP: HashMap<String, FeaturedWord> = make_map();
}

#[derive(Debug, Clone)]
pub enum Token {
    Plain(String),
    Punctuation(String),
    Comment(String),
}

impl Natural for Token {
    fn as_natural(&self) -> String {
        match self {
            Token::Plain(s) | Token::Punctuation(s) | Token::Comment(s) => s.clone(),
        }
    }
}

impl Token {
    pub fn content(&self) -> &str {
        match self {
            Token::Plain(s) | Token::Punctuation(s) | Token::Comment(s) => s,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenWithLocation {
    token: Token,
    line: usize,
    column: usize,
}

impl From<TokenWithLocation> for Token {
    fn from(value: TokenWithLocation) -> Self {
        value.token
    }
}

impl TokenWithLocation {
    pub fn peek_inner(&self) -> &Token {
        &self.token
    }
}

pub struct TokenizationState {
    buffer: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub enum TokenizationError {
    UnexpectedEndOfInput,
    InvalidCharacter {
        character: char,
        line: usize,
        column: usize,
    },
    UnknownWord {
        word: String,
        line: usize,
        column: usize,
    },
    ErrorInString {
        remainder: String,
        inner: Box<TokenizationError>,
    },
    NotEmptyBuffer(String),
}

impl TokenizationState {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            line: 1,
            column: 1,
        }
    }

    pub fn is_empty(&self) -> Result<(), TokenizationError> {
        if self.buffer.is_empty() {
            Ok(())
        } else {
            Err(TokenizationError::NotEmptyBuffer(self.buffer.clone()))
        }
    }

    fn commit_buffer(&mut self) -> Result<Vec<TokenWithLocation>, TokenizationError> {
        if self.buffer.is_empty() {
            Ok(Vec::new())
        } else if let Some(featured) = WORD_LOOKUP.get(&self.buffer) {
            let mut tokens = Vec::new();
            if let Some(prefix) = featured.prefix {
                tokens.push(TokenWithLocation {
                    token: Token::Plain(prefix.to_string()),
                    line: self.line,
                    column: self.column,
                });
            }
            tokens.push(TokenWithLocation {
                token: Token::Plain(featured.root.to_string()),
                line: self.line,
                column: self.column,
            });
            if let Some(suffix) = featured.suffix {
                tokens.push(TokenWithLocation {
                    token: Token::Plain(suffix.to_string()),
                    line: self.line,
                    column: self.column,
                });
            }
            if let Some(sufsuffix) = featured.sufsuffix {
                tokens.push(TokenWithLocation {
                    token: Token::Plain(sufsuffix.to_string()),
                    line: self.line,
                    column: self.column,
                });
            }
            self.buffer.clear();
            Ok(tokens)
        } else {
            Err(TokenizationError::UnknownWord {
                word: self.buffer.clone(),
                line: self.line,
                column: self.column,
            })
        }
    }

    pub fn feed_one(&mut self, c: char) -> Result<Vec<TokenWithLocation>, TokenizationError> {
        let mut tokens = Vec::new();
        if c.is_whitespace() {
            for t in self.commit_buffer()? {
                tokens.push(t);
            }
            self.column += 1;
            if c == '\n' {
                tokens.push(TokenWithLocation {
                    token: Token::Punctuation("\n".to_string()),
                    line: self.line,
                    column: self.column,
                });
                self.line += 1;
                self.column = 1;
            }
            Ok(tokens)
        } else if c == '.' || c == ':' || c == ',' {
            for t in self.commit_buffer()? {
                tokens.push(t);
            }
            if c != ',' {
                tokens.push(TokenWithLocation {
                    token: Token::Punctuation(c.to_string()),
                    line: self.line,
                    column: self.column,
                });
            } else {
                tokens.push(TokenWithLocation {
                    token: Token::Comment(c.to_string()),
                    line: self.line,
                    column: self.column,
                });
            }
            self.column += 1;
            Ok(tokens)
        } else if c.is_ascii_alphabetic() {
            self.buffer.push(c);
            self.column += 1;
            Ok(tokens)
        } else {
            Err(TokenizationError::InvalidCharacter {
                character: c,
                line: self.line,
                column: self.column,
            })
        }
    }

    pub fn feed(&mut self, input: &str) -> Result<Vec<TokenWithLocation>, TokenizationError> {
        let mut tokens = Vec::new();
        let mut pointer = 0;
        for c in input.chars() {
            match self.feed_one(c) {
                Ok(mut ts) => {
                    tokens.append(&mut ts);
                    pointer += c.len_utf8();
                }
                Err(e) => {
                    let remainder = input[pointer..].to_string();
                    return Err(TokenizationError::ErrorInString {
                        remainder,
                        inner: Box::new(e),
                    });
                }
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_short() {
        let mut tokenizer = TokenizationState::new();
        let input = "sama ijosu mita lonpala, semesu pali lonpa.";
        let tokens = tokenizer.feed(input).unwrap();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn test_long() {
        let mut tokenizer = TokenizationState::new();
        let input = include_str!("../../example.txt");
        let tokens = tokenizer.feed(input).unwrap();
        insta::assert_debug_snapshot!(tokens);
    }
}
