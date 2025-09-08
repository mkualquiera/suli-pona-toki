use std::result;

use crate::{
    Natural,
    content::{ContentParseError, ContentTree},
    preposition::{Preposition, PrepositionParser},
    tokens::{Token, TokenWithLocation},
};

#[derive(Debug)]
struct Subject {
    prepositions: Vec<Preposition>,
    content: ContentTree,
}

impl Natural for Subject {
    fn as_natural(&self) -> String {
        let prepositions: Vec<String> = self.prepositions.iter().map(|p| p.as_natural()).collect();
        format!("{} {}⚬", prepositions.join(", "), self.content.as_natural())
    }
}

#[derive(Debug)]
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

#[derive(Clone, Debug)]
struct Object {
    prepositions: Vec<Preposition>,
    content: ContentTree,
}

impl Natural for Object {
    fn as_natural(&self) -> String {
        let prepositions: Vec<String> = self.prepositions.iter().map(|p| p.as_natural()).collect();
        format!("{} {}◯ ", prepositions.join(" "), self.content.as_natural())
    }
}

struct ObjectParser {
    preposition_parser: PrepositionParser,
    prepositions: Vec<Preposition>,
}

#[derive(Debug)]
enum ObjectParseError {
    UnfinishedContent(ContentParseError),
    OrphanPrepositions(Vec<Preposition>),
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
    pub fn is_empty(&self) -> Result<(), ObjectParseError> {
        self.preposition_parser
            .is_empty()
            .map_err(ObjectParseError::UnfinishedContent)?;
        if self.prepositions.is_empty() {
            Ok(())
        } else {
            Err(ObjectParseError::OrphanPrepositions(
                self.prepositions.clone(),
            ))
        }
    }
}

#[derive(Debug)]
struct Verb {
    objects: Vec<Object>,
    prepositions: Vec<Preposition>,
    content: ContentTree,
}

impl Natural for Verb {
    fn as_natural(&self) -> String {
        let objects: Vec<String> = self.objects.iter().map(|o| o.as_natural()).collect();
        let prepositions: Vec<String> = self.prepositions.iter().map(|p| p.as_natural()).collect();
        format!(
            "{} {} {}⚡",
            objects.join(" "),
            prepositions.join(" "),
            self.content.as_natural()
        )
    }
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
    pub fn steal_objects(self) -> Result<Vec<Object>, ObjectParseError> {
        self.object_parser.is_empty()?;
        Ok(self.objects)
    }
}

#[derive(Debug)]
enum PredicateType {
    Verbed(Vec<Verb>),
    OrphanObjects(Vec<Object>),
}

impl Natural for PredicateType {
    fn as_natural(&self) -> String {
        match self {
            PredicateType::Verbed(verbs) => {
                let verbs: Vec<String> = verbs.iter().map(|v| v.as_natural()).collect();
                format!("{}^", verbs.join(" "))
            }
            PredicateType::OrphanObjects(objects) => {
                let objects: Vec<String> = objects.iter().map(|o| o.as_natural()).collect();
                format!("{}*", objects.join(" "))
            }
        }
    }
}

#[derive(Debug)]
struct Sentence {
    subject: Subject,
    predicate: PredicateType,
}

impl Natural for Sentence {
    fn as_natural(&self) -> String {
        format!(
            "{} {}§",
            self.subject.as_natural(),
            self.predicate.as_natural()
        )
    }
}

enum SentenceParser {
    ParsingSubject(SubjectParser),
    ParsingPredicate {
        subject: Subject,
        verbs: Vec<Verb>,
        verb_parser: VerbParser,
    },
}

enum SentenceParsingOutput {
    Continues(SentenceParser),
    Finished(Sentence),
}

#[derive(Debug)]
enum SentenceParseError {
    Content(ContentParseError),
    Object(ObjectParseError),
    OrphanedObjectsWithVerbs(Vec<Object>),
    UnfinishedSubject(SubjectParser),
}

impl SentenceParser {
    pub fn new() -> Self {
        Self::ParsingSubject(SubjectParser::new())
    }
    pub fn feed_one(
        self,
        token: TokenWithLocation,
    ) -> Result<SentenceParsingOutput, SentenceParseError> {
        match self {
            SentenceParser::ParsingSubject(subject_parser) => {
                if token.peek_inner().content() == "." {
                    return Err(SentenceParseError::UnfinishedSubject(subject_parser));
                }
                match subject_parser
                    .feed_one(token)
                    .map_err(SentenceParseError::Content)?
                {
                    SubjectParsingOutput::Continues(subject_parser) => {
                        Ok(SentenceParsingOutput::Continues(
                            SentenceParser::ParsingSubject(subject_parser),
                        ))
                    }
                    SubjectParsingOutput::Finished(subject) => Ok(
                        SentenceParsingOutput::Continues(SentenceParser::ParsingPredicate {
                            subject,
                            verb_parser: VerbParser::new(),
                            verbs: Vec::new(),
                        }),
                    ),
                }
            }
            SentenceParser::ParsingPredicate {
                subject,
                mut verb_parser,
                mut verbs,
            } => {
                if token.peek_inner().content() == "." {
                    let orphan_objects = verb_parser
                        .steal_objects()
                        .map_err(SentenceParseError::Object)?;
                    if orphan_objects.is_empty() {
                        Ok(SentenceParsingOutput::Finished(Sentence {
                            subject,
                            predicate: PredicateType::Verbed(verbs),
                        }))
                    } else if verbs.is_empty() {
                        Ok(SentenceParsingOutput::Finished(Sentence {
                            subject,
                            predicate: PredicateType::OrphanObjects(orphan_objects),
                        }))
                    } else {
                        Err(SentenceParseError::OrphanedObjectsWithVerbs(orphan_objects))
                    }
                } else {
                    let result = verb_parser
                        .feed_one(token)
                        .map_err(SentenceParseError::Content)?;
                    if let Some(verb) = result {
                        verbs.push(verb);
                    }
                    Ok(SentenceParsingOutput::Continues(
                        SentenceParser::ParsingPredicate {
                            subject,
                            verb_parser,
                            verbs,
                        },
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenizationState;

    #[test]
    fn test_sentence() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer
            .feed("utata misu okona mokumo ponalu lukinpa.")
            .unwrap();
        let mut parser = SentenceParser::new();
        let mut sentence = None;
        for token in tokens {
            let parsing_output = parser.feed_one(token).unwrap();
            match parsing_output {
                SentenceParsingOutput::Continues(p) => parser = p,
                SentenceParsingOutput::Finished(s) => {
                    sentence = Some(s);
                    break;
                }
            }
        }
        assert!(sentence.is_some());
        insta::assert_snapshot!(sentence.unwrap().as_natural());
    }
}
