use std::{
    error::Error,
    fmt::Display,
    io::{Read, Write},
};

use suli_pona_toki::{
    Natural,
    sentence::{self, SentenceParseError, SentenceParser, SentenceParsingOutput},
    tokens::{TokenizationError, TokenizationState},
};

use crate::html::AsHtml;

#[derive(Debug)]
pub enum TranspileError {
    ReadError(std::io::Error, (usize, usize)),
    UnmatchedHtmlTags((usize, usize)),
    WriteError(std::io::Error, (usize, usize)),
    TokenizationError(TokenizationError, (usize, usize)),
    ParsingError(SentenceParseError, (usize, usize)),
}

impl Display for TranspileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranspileError::ReadError(e, (line, col)) => {
                write!(f, "Error reading input at {}:{}: {}", line, col, e)
            }
            TranspileError::UnmatchedHtmlTags((line, col)) => {
                write!(f, "Unmatched HTML tags in input at {}:{}", line, col)
            }
            TranspileError::WriteError(e, (line, col)) => {
                write!(f, "Error writing output at {}:{}: {}", line, col, e)
            }
            TranspileError::TokenizationError(e, (line, col)) => {
                write!(f, "Tokenization error at {}:{}: {:?}", line, col, e)
            }
            TranspileError::ParsingError(e, (line, col)) => {
                write!(f, "Parsing error at {}:{}: {:?}", line, col, e)
            }
        }
    }
}

impl Error for TranspileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TranspileError::ReadError(e, _) => Some(e),
            TranspileError::UnmatchedHtmlTags(_) => None,
            TranspileError::WriteError(e, _) => Some(e),
            TranspileError::TokenizationError(e, _) => None,
            TranspileError::ParsingError(e, _) => None,
        }
    }
}

pub fn transpile_stream(
    input: &mut dyn Read,
    output: &mut dyn Write,
) -> Result<(), TranspileError> {
    let mut sentence_parser = SentenceParser::new();
    let mut tokenizer = TokenizationState::new();
    let mut html_depth = 0;

    let mut line = 1;
    let mut column = 1;

    let mut wrote_something = false;

    enum NewlineState {
        None,
        BrokeLine,
        NewParagraph,
    }

    let mut newline_state = NewlineState::NewParagraph;

    for byte in input.bytes() {
        column += 1;
        //if let Err(e) = byte {
        //    return Err(TranspileError::ReadError(e));
        //}
        //let byte = byte.unwrap();
        let byte = byte.map_err(|e| TranspileError::ReadError(e, (line, column)))?;
        let char = byte as char;
        if char == '\n' {
            line += 1;
            column = 1;
        }
        let mut do_write = false;
        if char == '<' {
            html_depth += 1;
            do_write = true;
            tokenizer
                .is_empty()
                .map_err(|e| TranspileError::TokenizationError(e, (line, column)))?;
            sentence_parser
                .is_empty()
                .map_err(|e| TranspileError::ParsingError(e, (line, column)))?;
            if html_depth == 1 && wrote_something {
                output
                    .write_all(b"</p>")
                    .map_err(|e| TranspileError::WriteError(e, (line, column)))?;
            }
        }
        if char == '>' {
            html_depth -= 1;
            do_write = true;
            wrote_something = false;
        }
        if html_depth < 0 {
            return Err(TranspileError::UnmatchedHtmlTags((line, column)));
        }
        if do_write || html_depth > 0 {
            //if let Err(e) = output.write_all(&[byte]) {
            //    return Err(TranspileError::WriteError(e));
            //}
            output
                .write_all(&[byte])
                .map_err(|e| TranspileError::WriteError(e, (line, column)))?;
        } else {
            // for now we just write 't' as a placeholder for parsed language
            //if let Err(e) = output.write_all(b"t") {
            //    return Err(TranspileError::WriteError(e));
            //}
            if char == '\n' {
                sentence_parser
                    .is_empty()
                    .map_err(|e| TranspileError::ParsingError(e, (line, column)))?;
                match newline_state {
                    NewlineState::None => {
                        newline_state = NewlineState::BrokeLine;
                    }
                    NewlineState::BrokeLine => {
                        newline_state = NewlineState::NewParagraph;
                        output
                            .write_all(b"</p>")
                            .map_err(|e| TranspileError::WriteError(e, (line, column)))?;
                    }
                    NewlineState::NewParagraph => {}
                }
                continue;
            }
            let tokens = tokenizer
                .feed_one(char)
                .map_err(|e| TranspileError::TokenizationError(e, (line, column)))?;
            for token in tokens {
                wrote_something = true;
                let sentence_result = sentence_parser
                    .feed_one(token)
                    .map_err(|e| TranspileError::ParsingError(e, (line, column)))?;
                match sentence_result {
                    SentenceParsingOutput::Continues(parser) => {
                        sentence_parser = parser;
                    }
                    SentenceParsingOutput::Finished(sentence) => {
                        match newline_state {
                            NewlineState::None => {}
                            NewlineState::BrokeLine => {
                                output
                                    .write_all(b"<br/>")
                                    .map_err(|e| TranspileError::WriteError(e, (line, column)))?;
                                newline_state = NewlineState::None;
                            }
                            NewlineState::NewParagraph => {
                                output
                                    .write_all(b"<p>")
                                    .map_err(|e| TranspileError::WriteError(e, (line, column)))?;
                                newline_state = NewlineState::None;
                            }
                        }
                        //let _ = write!(output, "[[Parsed sentence: {}]]", sentence.as_natural());
                        sentence
                            .write_html(output, None)
                            .map_err(|e| TranspileError::WriteError(e, (line, column)))?;
                        sentence_parser = SentenceParser::new();
                    }
                }
            }
        }
    }
    tokenizer
        .is_empty()
        .map_err(|e| TranspileError::TokenizationError(e, (line, column)))?;
    sentence_parser
        .is_empty()
        .map_err(|e| TranspileError::ParsingError(e, (line, column)))?;
    Ok(())
}
