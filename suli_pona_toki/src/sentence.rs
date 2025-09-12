use std::result;

use crate::{
    Natural,
    content::{ContentParseError, ContentTree},
    preposition::{Preposition, PrepositionParser},
    tokens::{Token, TokenWithLocation},
};

trait Upgrade<T> {
    fn upgrade(self) -> T;
}

#[derive(Debug)]
pub struct Subject {
    prepositions: Vec<Preposition>,
    content: ContentTree,
}

impl Natural for Subject {
    fn as_natural(&self) -> String {
        let prepositions: Vec<String> = self.prepositions.iter().map(|p| p.as_natural()).collect();
        format!("{} {}⚬", prepositions.join(" "), self.content.as_natural())
    }
}

#[derive(Debug)]
pub struct SubjectParser {
    preposition_parser: PrepositionParser,
    prepositions: Vec<Preposition>,
}

pub enum SubjectParsingOutput {
    Continues(SubjectParser),
    Finished(Subject),
}

#[derive(Debug)]
pub enum SubjectParsingError {
    PrepositionsNotEmpty(Vec<Preposition>),
    HasContent(ContentParseError),
}

impl SubjectParser {
    pub fn new() -> Self {
        Self {
            preposition_parser: PrepositionParser::new(),
            prepositions: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> Result<(), SubjectParsingError> {
        self.preposition_parser
            .is_empty()
            .map_err(SubjectParsingError::HasContent)?;
        if self.prepositions.is_empty() {
            Ok(())
        } else {
            Err(SubjectParsingError::PrepositionsNotEmpty(
                self.prepositions.clone(),
            ))
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
pub struct Object {
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
pub enum ObjectParseError {
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

impl Upgrade<ObjectParser> for SubjectParser {
    fn upgrade(self) -> ObjectParser {
        let SubjectParser {
            preposition_parser,
            prepositions,
        } = self;
        ObjectParser {
            preposition_parser,
            prepositions,
        }
    }
}

#[derive(Debug)]
pub struct Verb {
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

pub struct VerbParser {
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

impl Upgrade<VerbParser> for SubjectParser {
    fn upgrade(self) -> VerbParser {
        VerbParser {
            object_parser: self.upgrade(),
            objects: Vec::new(),
        }
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
pub struct Sentence {
    context: Option<Box<Sentence>>,
    subject: Option<Subject>,
    predicate: PredicateType,
    apposition: Option<Box<Sentence>>,
}

impl Natural for Sentence {
    fn as_natural(&self) -> String {
        let ending = if self.subject.is_some() { "+§" } else { "-§" };
        let subject = if let Some(subject) = &self.subject {
            subject.as_natural()
        } else {
            String::new()
        };
        let context = if let Some(context) = &self.context {
            format!("{} | ", context.as_natural())
        } else {
            String::new()
        };
        let apposition = if let Some(apposition) = &self.apposition {
            format!(" : {}", apposition.as_natural())
        } else {
            String::new()
        };
        format!(
            "{} {} {}{}{}",
            context,
            subject,
            self.predicate.as_natural(),
            apposition,
            ending
        )
    }
}

pub enum SentenceParser {
    ParsingSubject {
        context: Option<Box<Sentence>>,
        subject_parser: SubjectParser,
    },
    ParsingPredicate {
        context: Option<Box<Sentence>>,
        subject: Option<Subject>,
        verbs: Vec<Verb>,
        verb_parser: VerbParser,
        apposition_parser: Option<Box<SentenceParser>>,
    },
}

pub enum SentenceParsingOutput {
    Continues(SentenceParser),
    Finished(Sentence),
}

#[derive(Debug)]
pub enum SentenceParseError {
    Content(ContentParseError),
    Object(ObjectParseError),
    OrphanedObjectsWithVerbs(Vec<Object>, TokenWithLocation),
    UnfinishedSubject(SubjectParser, TokenWithLocation),
    EmptySentence(TokenWithLocation),
    AppositionInSubject(TokenWithLocation),
    PredicateNotEmpty,
    ContextNotEmpty,
    SubjectNotEmpty(SubjectParsingError),
}

impl SentenceParser {
    pub fn new() -> Self {
        Self::ParsingSubject {
            context: None,
            subject_parser: SubjectParser::new(),
        }
    }
    pub fn is_empty(&mut self) -> Result<(), SentenceParseError> {
        match self {
            SentenceParser::ParsingSubject {
                context,
                subject_parser,
            } => {
                if context.is_some() {
                    return Err(SentenceParseError::ContextNotEmpty);
                }
                subject_parser
                    .is_empty()
                    .map_err(SentenceParseError::SubjectNotEmpty)
            }
            SentenceParser::ParsingPredicate { .. } => Err(SentenceParseError::PredicateNotEmpty),
        }
    }
    pub fn feed_one(
        self,
        token: TokenWithLocation,
    ) -> Result<SentenceParsingOutput, SentenceParseError> {
        match self {
            SentenceParser::ParsingSubject {
                context,
                subject_parser,
            } => {
                if token.peek_inner().content() == ":" {
                    return Err(SentenceParseError::AppositionInSubject(token));
                }
                if token.peek_inner().content() == "mo" || token.peek_inner().content() == "pa" {
                    let verb_parser = subject_parser.upgrade();
                    let new_sentence_parser = SentenceParser::ParsingPredicate {
                        context,
                        subject: None,
                        verb_parser,
                        verbs: Vec::new(),
                        apposition_parser: None,
                    };
                    return new_sentence_parser.feed_one(token);
                }
                if token.peek_inner().content() == "." || token.peek_inner().content() == "la" {
                    return Err(SentenceParseError::UnfinishedSubject(subject_parser, token));
                }
                match subject_parser
                    .feed_one(token)
                    .map_err(SentenceParseError::Content)?
                {
                    SubjectParsingOutput::Continues(subject_parser) => Ok(
                        SentenceParsingOutput::Continues(SentenceParser::ParsingSubject {
                            context,
                            subject_parser,
                        }),
                    ),
                    SubjectParsingOutput::Finished(subject) => Ok(
                        SentenceParsingOutput::Continues(SentenceParser::ParsingPredicate {
                            context,
                            subject: Some(subject),
                            verb_parser: VerbParser::new(),
                            verbs: Vec::new(),
                            apposition_parser: None,
                        }),
                    ),
                }
            }
            SentenceParser::ParsingPredicate {
                context,
                subject,
                mut verb_parser,
                mut verbs,
                mut apposition_parser,
            } => {
                let inner_token = token.clone();
                let commit_sentence =
                    |context,
                     subject: Option<Subject>,
                     verb_parser: VerbParser,
                     verbs: Vec<Verb>,
                     apposition: Option<Box<Sentence>>| {
                        let orphan_objects = verb_parser
                            .steal_objects()
                            .map_err(SentenceParseError::Object)?;
                        if orphan_objects.is_empty() && verbs.is_empty() && subject.is_none() {
                            let token = inner_token.clone();
                            Err(SentenceParseError::EmptySentence(token))
                        } else if orphan_objects.is_empty() {
                            Ok(Sentence {
                                context,
                                subject,
                                predicate: PredicateType::Verbed(verbs),
                                apposition,
                            })
                        } else if verbs.is_empty() {
                            Ok(Sentence {
                                context,
                                subject,
                                predicate: PredicateType::OrphanObjects(orphan_objects),
                                apposition,
                            })
                        } else {
                            let token = inner_token.clone();
                            Err(SentenceParseError::OrphanedObjectsWithVerbs(
                                orphan_objects,
                                token,
                            ))
                        }
                    };
                if let Some(apposition_parser) = apposition_parser.take() {
                    let aposition_parsing_output = apposition_parser.feed_one(token)?;
                    match aposition_parsing_output {
                        SentenceParsingOutput::Finished(apposition) => {
                            return Ok(SentenceParsingOutput::Finished(commit_sentence(
                                context,
                                subject,
                                verb_parser,
                                verbs,
                                Some(Box::new(apposition)),
                            )?));
                        }
                        SentenceParsingOutput::Continues(p) => {
                            return Ok(SentenceParsingOutput::Continues(
                                SentenceParser::ParsingPredicate {
                                    context,
                                    subject,
                                    verb_parser,
                                    verbs,
                                    apposition_parser: Some(Box::new(p)),
                                },
                            ));
                        }
                    }
                }
                if token.peek_inner().content() == ":" {
                    if apposition_parser.is_some() {
                        unreachable!("Multiple appositions are not allowed");
                    }
                    let apposition_parser = Box::new(SentenceParser::new());
                    return Ok(SentenceParsingOutput::Continues(
                        SentenceParser::ParsingPredicate {
                            context,
                            subject,
                            verb_parser,
                            verbs,
                            apposition_parser: Some(apposition_parser),
                        },
                    ));
                }
                if token.peek_inner().content() == "." {
                    Ok(SentenceParsingOutput::Finished(commit_sentence(
                        context,
                        subject,
                        verb_parser,
                        verbs,
                        None,
                    )?))
                } else if token.peek_inner().content() == "la" {
                    let context_sentence =
                        Box::new(commit_sentence(context, subject, verb_parser, verbs, None)?);
                    Ok(SentenceParsingOutput::Continues(
                        SentenceParser::ParsingSubject {
                            context: Some(context_sentence),
                            subject_parser: SubjectParser::new(),
                        },
                    ))
                } else {
                    let result = verb_parser
                        .feed_one(token)
                        .map_err(SentenceParseError::Content)?;
                    if let Some(verb) = result {
                        verbs.push(verb);
                    }
                    Ok(SentenceParsingOutput::Continues(
                        SentenceParser::ParsingPredicate {
                            context,
                            subject,
                            verb_parser,
                            verbs,
                            apposition_parser: None,
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

    #[test]
    fn test_sentence_with_context() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer
            .feed("utata misula tokisu okona mokumo ponalu lukinpa.")
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

    #[test]
    fn test_subjectless_sentence() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("okomola tawapa.").unwrap();
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

    #[test]
    fn test_apposition() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("nisu ponapa: ijomo mokupa.").unwrap();
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
