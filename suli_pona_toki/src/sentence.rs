use std::result;

use crate::{
    content::{ContentParseError, ContentTree},
    preposition::{Preposition, PrepositionParser},
    tokens::{Token, TokenWithLocation},
};

struct Subject {
    prepositions: Vec<Preposition>,
    content: ContentTree,
}

struct SubjectParser {
    preposition_parser: PrepositionParser,
    prepositions: Vec<Preposition>,
}

enum SubjectParsingOutput {
    Continues(SubjectParser),
    Finished(Subject),
}

impl SubjectParser {
    pub fn new() -> Self {
        Self {
            preposition_parser: PrepositionParser::new(),
            prepositions: Vec::new(),
        }
    }
    pub fn feed_one(
        mut self,
        token: TokenWithLocation,
    ) -> Result<SubjectParsingOutput, ContentParseError> {
        if token.peek_inner().content() == "su" {
            Ok(SubjectParsingOutput::Finished(Subject {
                prepositions: self.prepositions,
                content: self.preposition_parser.close()?,
            }))
        } else {
            let result = self.preposition_parser.feed_one(token)?;
            if let Some(preposition) = result {
                self.prepositions.push(preposition);
            }
            Ok(SubjectParsingOutput::Continues(self))
        }
    }
}
