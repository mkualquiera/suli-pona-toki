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

#[derive(Clone)]
struct Object {
    prepositions: Vec<Preposition>,
    content: ContentTree,
}

struct ObjectParser {
    preposition_parser: PrepositionParser,
    prepositions: Vec<Preposition>,
}

impl ObjectParser {
    pub fn new() -> Self {
        Self {
            preposition_parser: PrepositionParser::new(),
            prepositions: Vec::new(),
        }
    }
    pub fn feed_one(
        &mut self,
        token: TokenWithLocation,
    ) -> Result<Option<Object>, ContentParseError> {
        if token.peek_inner().content() == "mo" {
            let result = Ok(Some(Object {
                prepositions: self.prepositions.clone(),
                content: self.preposition_parser.take()?,
            }));
            self.prepositions.clear();
            result
        } else {
            let result = self.preposition_parser.feed_one(token)?;
            if let Some(preposition) = result {
                self.prepositions.push(preposition);
            }
            Ok(None)
        }
    }
    pub fn take(&mut self) -> Result<(ContentTree, Vec<Preposition>), ContentParseError> {
        let content = self.preposition_parser.take()?;
        let prepositions = self.prepositions.clone();
        self.prepositions.clear();
        Ok((content, prepositions))
    }
}

struct Verb {
    objects: Vec<Object>,
    prepositions: Vec<Preposition>,
    content: ContentTree,
}

struct VerbParser {
    object_parser: ObjectParser,
    objects: Vec<Object>,
}

impl VerbParser {
    pub fn new() -> Self {
        Self {
            object_parser: ObjectParser::new(),
            objects: Vec::new(),
        }
    }
    pub fn feed_one(
        &mut self,
        token: TokenWithLocation,
    ) -> Result<Option<Verb>, ContentParseError> {
        if token.peek_inner().content() == "pa" {
            let (content, prepositions) = self.object_parser.take()?;
            let result = Ok(Some(Verb {
                objects: self.objects.clone(),
                prepositions,
                content,
            }));
            self.objects.clear();
            result
        } else {
            let result = self.object_parser.feed_one(token)?;
            if let Some(object) = result {
                self.objects.push(object);
            }
            Ok(None)
        }
    }
}
