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

#[derive(Debug)]
pub enum TranspileError {
    ReadError(std::io::Error),
    UnmatchedHtmlTags,
    WriteError(std::io::Error),
    TokenizationError(TokenizationError),
    ParsingError(SentenceParseError),
}

impl Display for TranspileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranspileError::ReadError(e) => write!(f, "Error reading input: {}", e),
            TranspileError::UnmatchedHtmlTags => write!(f, "Unmatched HTML tags in input"),
            TranspileError::WriteError(e) => write!(f, "Error writing output: {}", e),
            TranspileError::TokenizationError(e) => write!(f, "Tokenization error: {:?}", e),
            TranspileError::ParsingError(e) => write!(f, "Parsing error: {:?}", e),
        }
    }
}

impl Error for TranspileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TranspileError::ReadError(e) => Some(e),
            TranspileError::UnmatchedHtmlTags => None,
            TranspileError::WriteError(e) => Some(e),
            TranspileError::TokenizationError(e) => None,
            TranspileError::ParsingError(e) => None,
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

    let mut wrote_something = false;

    enum NewlineState {
        None,
        BrokeLine,
        NewParagraph,
    }

    let mut newline_state = NewlineState::NewParagraph;

    for byte in input.bytes() {
        //if let Err(e) = byte {
        //    return Err(TranspileError::ReadError(e));
        //}
        //let byte = byte.unwrap();
        let byte = byte.map_err(TranspileError::ReadError)?;
        let char = byte as char;
        let mut do_write = false;
        if char == '<' {
            html_depth += 1;
            do_write = true;
            tokenizer
                .is_empty()
                .map_err(TranspileError::TokenizationError)?;
            sentence_parser
                .is_empty()
                .map_err(TranspileError::ParsingError)?;
            if html_depth == 1 && wrote_something {
                output
                    .write_all(b"</p>")
                    .map_err(TranspileError::WriteError)?;
            }
        }
        if char == '>' {
            html_depth -= 1;
            do_write = true;
            wrote_something = false;
        }
        if html_depth < 0 {
            return Err(TranspileError::UnmatchedHtmlTags);
        }
        if do_write || html_depth > 0 {
            //if let Err(e) = output.write_all(&[byte]) {
            //    return Err(TranspileError::WriteError(e));
            //}
            output
                .write_all(&[byte])
                .map_err(TranspileError::WriteError)?;
        } else {
            // for now we just write 't' as a placeholder for parsed language
            //if let Err(e) = output.write_all(b"t") {
            //    return Err(TranspileError::WriteError(e));
            //}
            if char == '\n' {
                sentence_parser
                    .is_empty()
                    .map_err(TranspileError::ParsingError)?;
                match newline_state {
                    NewlineState::None => {
                        newline_state = NewlineState::BrokeLine;
                    }
                    NewlineState::BrokeLine => {
                        newline_state = NewlineState::NewParagraph;
                        output
                            .write_all(b"</p>")
                            .map_err(TranspileError::WriteError)?;
                    }
                    NewlineState::NewParagraph => {}
                }
                continue;
            }
            let tokens = tokenizer
                .feed_one(char)
                .map_err(TranspileError::TokenizationError)?;
            for token in tokens {
                wrote_something = true;
                let sentence_result = sentence_parser
                    .feed_one(token)
                    .map_err(TranspileError::ParsingError)?;
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
                                    .map_err(TranspileError::WriteError)?;
                                newline_state = NewlineState::None;
                            }
                            NewlineState::NewParagraph => {
                                output
                                    .write_all(b"<p>")
                                    .map_err(TranspileError::WriteError)?;
                                newline_state = NewlineState::None;
                            }
                        }
                        let _ = write!(output, "[[Parsed sentence: {}]]", sentence.as_natural());
                        sentence_parser = SentenceParser::new();
                    }
                }
            }
        }
    }
    Ok(())
}
