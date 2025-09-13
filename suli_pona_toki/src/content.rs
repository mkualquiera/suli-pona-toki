use crate::{
    Natural,
    tokens::{Token, TokenWithLocation},
};

#[derive(Debug, Clone)]
pub enum LeafType {
    Literal(Vec<Token>),
    Core(Token),
}

impl Natural for LeafType {
    fn as_natural(&self) -> String {
        match self {
            LeafType::Literal(tokens) => {
                let parts: Vec<String> = tokens.iter().map(|t| t.as_natural()).collect();
                format!("[{}]", parts.join(" "))
            }
            LeafType::Core(token) => token.as_natural(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchType {
    Concatenation,
    Property,
    InnerGlue,
    Union,
    Alternative,
}

#[derive(Debug, Clone)]
pub enum ContentTree {
    Terminal,
    Leaf(LeafType),
    Branch {
        branch_type: BranchType,
        head: Box<ContentTree>,
        tail: Box<ContentTree>,
    },
}

impl Natural for ContentTree {
    fn as_natural(&self) -> String {
        match self {
            ContentTree::Terminal => "∅".into(),
            ContentTree::Leaf(leaf) => leaf.as_natural(),
            ContentTree::Branch {
                branch_type,
                head,
                tail,
            } => {
                format!("({} {})", tail.as_natural(), head.as_natural())
            }
        }
    }
}

impl ContentTree {
    fn is_valid(&self) -> bool {
        match self {
            ContentTree::Terminal => false,
            ContentTree::Leaf(_) => true,
            ContentTree::Branch { head, tail, .. } => {
                // Head cannot be Terminal in a valid tree
                !matches!(head.as_ref(), ContentTree::Terminal)
                    && head.is_valid()
                    && tail.is_valid()
            }
        }
    }
}

#[derive(Debug)]
pub enum ContentParseError {
    NotPlainToken(TokenWithLocation),
    GlueWithoutBranch(TokenWithLocation),
    PropertyOfNothing(TokenWithLocation),
    AlternativeOfNothing(TokenWithLocation),
    UnionOfNothing(TokenWithLocation),
    InvalidTreeStructure(ContentTree),
    ErrorInString {
        remainder: Vec<TokenWithLocation>,
        inner: Box<ContentParseError>,
        state: ContentTree,
    },
    NotEmpty(ContentTree),
}

#[derive(Debug)]
pub struct ContentTreeParser {
    state: ContentTree,
}

impl ContentTreeParser {
    pub fn new() -> Self {
        Self {
            state: ContentTree::Terminal,
        }
    }

    pub fn is_empty(&self) -> Result<(), ContentParseError> {
        if let ContentTree::Terminal = self.state {
            Ok(())
        } else {
            Err(ContentParseError::NotEmpty(self.state.clone()))
        }
    }

    pub fn feed_one(&mut self, token: TokenWithLocation) -> Result<(), ContentParseError> {
        let data = match token.peek_inner() {
            Token::Plain(data) => data,
            Token::Comment(_) => return Ok(()), // Ignore comments
            _ => return Err(ContentParseError::NotPlainToken(token)),
        };

        self.state = match data.as_str() {
            "un" => {
                if let ContentTree::Terminal = self.state {
                    return Err(ContentParseError::UnionOfNothing(token));
                } else {
                    ContentTree::Branch {
                        branch_type: BranchType::Union,
                        head: Box::new(ContentTree::Terminal),
                        tail: Box::new(self.state.clone()),
                    }
                }
            }
            "nu" => {
                if let ContentTree::Terminal = self.state {
                    return Err(ContentParseError::AlternativeOfNothing(token));
                } else {
                    ContentTree::Branch {
                        branch_type: BranchType::Alternative,
                        head: Box::new(ContentTree::Terminal),
                        tail: Box::new(self.state.clone()),
                    }
                }
            }
            "pi" => {
                if let ContentTree::Terminal = self.state {
                    return Err(ContentParseError::PropertyOfNothing(token));
                } else {
                    ContentTree::Branch {
                        branch_type: BranchType::Property,
                        head: Box::new(ContentTree::Terminal),
                        tail: Box::new(self.state.clone()),
                    }
                }
            }
            "no" => {
                if let ContentTree::Branch {
                    branch_type,
                    head,
                    tail,
                } = self.state.clone()
                {
                    ContentTree::Branch {
                        branch_type,
                        head: Box::new(ContentTree::Branch {
                            branch_type: BranchType::InnerGlue,
                            head: Box::new(ContentTree::Terminal),
                            tail: head,
                        }),
                        tail,
                    }
                } else {
                    return Err(ContentParseError::GlueWithoutBranch(token));
                }
            }
            _ => match self.state.clone() {
                ContentTree::Terminal => {
                    if data.to_uppercase() == *data {
                        ContentTree::Leaf(LeafType::Literal(vec![token.into()]))
                    } else {
                        ContentTree::Leaf(LeafType::Core(token.into()))
                    }
                }
                ContentTree::Leaf(leaf_type) => match leaf_type {
                    LeafType::Literal(mut tokens) => {
                        if data.to_uppercase() == *data {
                            tokens.push(token.into());
                            ContentTree::Leaf(LeafType::Literal(tokens))
                        } else {
                            ContentTree::Branch {
                                branch_type: BranchType::Concatenation,
                                head: Box::new(ContentTree::Leaf(LeafType::Core(token.into()))),
                                tail: Box::new(self.state.clone()),
                            }
                        }
                    }
                    LeafType::Core(_) => {
                        if data.to_uppercase() == *data {
                            ContentTree::Branch {
                                branch_type: BranchType::Concatenation,
                                head: Box::new(ContentTree::Leaf(LeafType::Literal(vec![
                                    token.into(),
                                ]))),
                                tail: Box::new(self.state.clone()),
                            }
                        } else {
                            ContentTree::Branch {
                                branch_type: BranchType::Concatenation,
                                head: Box::new(ContentTree::Leaf(LeafType::Core(token.into()))),
                                tail: Box::new(self.state.clone()),
                            }
                        }
                    }
                },
                ContentTree::Branch {
                    branch_type,
                    mut head,
                    tail,
                } => {
                    // Create a temporary parser for the head
                    let mut head_parser = ContentTreeParser { state: *head };
                    head_parser.feed_one(token)?;
                    head = Box::new(head_parser.state);

                    ContentTree::Branch {
                        branch_type,
                        head,
                        tail,
                    }
                }
            },
        };

        Ok(())
    }

    pub fn feed(&mut self, mut tokens: Vec<TokenWithLocation>) -> Result<(), ContentParseError> {
        while !tokens.is_empty() {
            let token = tokens.remove(0);
            if let Err(e) = self.feed_one(token) {
                return Err(ContentParseError::ErrorInString {
                    remainder: tokens,
                    inner: Box::new(e),
                    state: self.state.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn close(self) -> Result<ContentTree, ContentParseError> {
        if self.state.is_valid() {
            Ok(self.state)
        } else {
            Err(ContentParseError::InvalidTreeStructure(self.state))
        }
    }

    pub fn take(&mut self) -> Result<ContentTree, ContentParseError> {
        if self.state.is_valid() {
            let state = self.state.clone();
            self.state = ContentTree::Terminal;
            Ok(state)
        } else {
            Err(ContentParseError::InvalidTreeStructure(self.state.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenizationState;

    #[test]
    fn test_easy() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("sina suli mun luka linja ").unwrap();
        let mut parser = ContentTreeParser::new();
        parser.feed(tokens).unwrap();
        let tree = parser.close().unwrap();
        insta::assert_snapshot!(tree.as_natural());
    }

    #[test]
    fn test_medium() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("sina pona jan pi sitelen ilo ").unwrap();
        let mut parser = ContentTreeParser::new();
        parser.feed(tokens).unwrap();
        let tree = parser.close().unwrap();
        insta::assert_snapshot!(tree.as_natural());
    }

    #[test]
    fn test_hard() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer
            .feed("nasa telo pi ike jan no suli tomo ")
            .unwrap();
        let mut parser = ContentTreeParser::new();
        parser.feed(tokens).unwrap();
        let tree = parser.close().unwrap();
        insta::assert_snapshot!(tree.as_natural());
    }

    #[test]
    fn test_hardest() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer
            .feed("nasa telo nu loje telo pi PA PU TO JAN no ike jan no suli tomo un tawa tomo ")
            .unwrap();
        let mut parser = ContentTreeParser::new();
        parser.feed(tokens).unwrap();
        let tree = parser.close().unwrap();
        insta::assert_snapshot!(tree.as_natural());
    }

    #[test]
    fn test_invalid_tree() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("sina pi ").unwrap();
        let mut parser = ContentTreeParser::new();
        parser.feed(tokens).unwrap();

        // This should fail validation because pi creates a branch with Terminal head
        assert!(matches!(
            parser.close(),
            Err(ContentParseError::InvalidTreeStructure(_))
        ));
    }

    #[test]
    fn test_problematic() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("nasa telo").unwrap();
        let mut parser = ContentTreeParser::new();
        parser.feed(tokens).unwrap();
        let tree = parser.close().unwrap();
        insta::assert_snapshot!(tree.as_natural());
    }
}
