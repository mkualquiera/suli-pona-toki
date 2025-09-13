use std::io::Write;

use suli_pona_toki::{
    Natural,
    content::{BranchType, ContentTree, LeafType},
    preposition::{Preposition, PrepositionType},
    sentence::{Object, PredicateType, Sentence, Subject, Verb},
};

pub struct LeafState {
    negated: bool,
}

impl AsHtml for LeafType {
    type Context = LeafState;
    fn write_html(
        &self,
        output: &mut dyn Write,
        context: Option<Self::Context>,
    ) -> std::io::Result<()> {
        if context.expect("LeafState context is required").negated {
            write!(output, "<contentleaf type=\"negated\">")?;
        } else {
            write!(output, "<contentleaf>")?;
        }
        match self {
            LeafType::Literal(tokens) => {
                write!(
                    output,
                    "{}",
                    tokens.iter().map(|t| t.as_natural()).collect::<String>()
                )?;
            }
            LeafType::Core(token) => {
                write!(output, "{}", token.as_natural())?;
            }
        }
        write!(output, "</contentleaf>")?;
        Ok(())
    }
}

pub trait AsHtml {
    type Context;
    fn write_html(
        &self,
        output: &mut dyn Write,
        context: Option<Self::Context>,
    ) -> std::io::Result<()>;
}

impl AsHtml for ContentTree {
    type Context = ();
    fn write_html(
        &self,
        output: &mut dyn Write,
        _context: Option<Self::Context>,
    ) -> std::io::Result<()> {
        match self {
            ContentTree::Terminal { .. } => unreachable!(),
            ContentTree::Leaf { leaf_type, negated } => {
                leaf_type.write_html(output, Some(LeafState { negated: *negated }))?;
            }
            ContentTree::Branch {
                branch_type,
                head,
                tail,
            } => match branch_type {
                BranchType::Concatenation => {
                    tail.write_html(output, None)?;
                    head.write_html(output, None)?;
                }
                BranchType::Property => {
                    write!(output, "<property>")?;
                    tail.write_html(output, None)?;
                    write!(output, "</property>")?;
                    head.write_html(output, None)?;
                }
                BranchType::InnerGlue => {
                    write!(output, "<glue>")?;
                    write!(output, "<gluetail>")?;
                    tail.write_html(output, None)?;
                    write!(output, "</gluetail>")?;
                    write!(output, "<gluehead>")?;
                    head.write_html(output, None)?;
                    write!(output, "</gluehead>")?;
                    write!(output, "</glue>")?;
                }
                BranchType::Union => {
                    write!(output, "<union>")?;
                    tail.write_html(output, None)?;
                    write!(output, "</union>")?;
                    head.write_html(output, None)?;
                }
                BranchType::Alternative => {
                    write!(output, "<alternative>")?;
                    tail.write_html(output, None)?;
                    write!(output, "</alternative>")?;
                    head.write_html(output, None)?;
                }
            },
        }
        Ok(())
    }
}

impl AsHtml for Preposition {
    type Context = ();
    fn write_html(
        &self,
        output: &mut dyn Write,
        _context: Option<Self::Context>,
    ) -> std::io::Result<()> {
        write!(
            output,
            "<preposition type=\"{}\">",
            match self.preposition_type {
                PrepositionType::Direction => "direction",
                PrepositionType::Origin => "origin",
                PrepositionType::Using => "using",
                PrepositionType::Location => "location",
                PrepositionType::Metacommentary => "metacommentary",
                PrepositionType::Manner => "manner",
            }
        )?;
        self.content.write_html(output, None)?;
        write!(output, "</preposition>")?;
        Ok(())
    }
}

impl AsHtml for Subject {
    type Context = ();
    fn write_html(
        &self,
        output: &mut dyn Write,
        _context: Option<Self::Context>,
    ) -> std::io::Result<()> {
        write!(output, "<subject>")?;
        for preposition in &self.prepositions {
            preposition.write_html(output, None)?;
        }
        self.content.write_html(output, None)?;
        write!(output, "</subject>")?;
        Ok(())
    }
}

impl AsHtml for Object {
    type Context = ();
    fn write_html(
        &self,
        output: &mut dyn Write,
        _context: Option<Self::Context>,
    ) -> std::io::Result<()> {
        write!(output, "<object>")?;
        for preposition in &self.prepositions {
            preposition.write_html(output, None)?;
        }
        self.content.write_html(output, None)?;
        write!(output, "</object>")?;
        Ok(())
    }
}

impl AsHtml for Verb {
    type Context = ();
    fn write_html(
        &self,
        output: &mut dyn Write,
        _context: Option<Self::Context>,
    ) -> std::io::Result<()> {
        write!(output, "<verb>")?;
        for object in &self.objects {
            object.write_html(output, None)?;
        }
        for preposition in &self.prepositions {
            preposition.write_html(output, None)?;
        }
        self.content.write_html(output, None)?;
        write!(output, "</verb>")?;
        Ok(())
    }
}

pub enum SentenceHTMLContext {
    Context,
    Apposition,
}

impl AsHtml for Sentence {
    type Context = SentenceHTMLContext;
    fn write_html(
        &self,
        output: &mut dyn Write,
        config: Option<Self::Context>,
    ) -> std::io::Result<()> {
        match config {
            Some(SentenceHTMLContext::Context) => write!(output, "<sentence role=\"context\">")?,
            Some(SentenceHTMLContext::Apposition) => {
                write!(output, "<sentence role=\"apposition\">")?
            }
            None => {
                write!(output, "<sentence>")?;
            }
        }
        if let Some(context) = &self.context {
            context.write_html(output, Some(SentenceHTMLContext::Context))?;
        }
        if let Some(subject) = &self.subject {
            subject.write_html(output, None)?;
        }
        //self.predicate.write_html(output)?;
        match self.predicate {
            PredicateType::Verbed(ref verbs) => {
                for verb in verbs {
                    verb.write_html(output, None)?;
                }
            }
            PredicateType::OrphanObjects(ref objects) => {
                for object in objects {
                    object.write_html(output, None)?;
                }
            }
        }
        if let Some(apposition) = &self.apposition {
            apposition.write_html(output, Some(SentenceHTMLContext::Apposition))?;
        }
        write!(output, "</sentence>")?;
        Ok(())
    }
}
