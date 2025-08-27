use crate::tokens::{Token, TokenWithLocation};

#[derive(Debug, Clone)]
pub enum LeafType {
    Literal(Vec<Token>),
    Core(Token),
}

impl LeafType {
    fn as_natural(&self) -> String {
        match self {
            LeafType::Literal(tokens) => {
                let parts: Vec<String> = tokens.iter().map(|t| format!("{:?}", t)).collect();
                format!("[{}]", parts.join(", "))
            }
            LeafType::Core(token) => format!("{:?}", token),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchType {
    Concatenation(Token),
    Property(Token),
    InnerGlue(Token),
    Union(Token),
    Alternative(Token),
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

impl ContentTree {
    fn as_natural(&self) -> String {
        match self {
            ContentTree::Terminal => "Terminal".into(),
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

    fn is_valid(&self) -> bool {
        match self {
            ContentTree::Terminal => true,
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
}

pub struct ContentTreeParser {
    state: ContentTree,
}

impl ContentTreeParser {
    pub fn new() -> Self {
        Self {
            state: ContentTree::Terminal,
        }
    }

    pub fn feed_one(&mut self, token: TokenWithLocation) -> Result<(), ContentParseError> {
        let data = if let Token::Plain(data) = token.clone().into() {
            data
        } else {
            return Err(ContentParseError::NotPlainToken(token));
        };

        self.state = match data.as_str() {
            "un" => {
                if let ContentTree::Terminal = self.state {
                    return Err(ContentParseError::UnionOfNothing(token));
                } else {
                    ContentTree::Branch {
                        branch_type: BranchType::Union(token.into()),
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
                        branch_type: BranchType::Alternative(token.into()),
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
                        branch_type: BranchType::Property(token.into()),
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
                        branch_type: BranchType::InnerGlue(token.into()),
                        head: Box::new(ContentTree::Branch {
                            branch_type,
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
                    if data.to_uppercase() == data {
                        ContentTree::Leaf(LeafType::Literal(vec![token.into()]))
                    } else {
                        ContentTree::Leaf(LeafType::Core(token.into()))
                    }
                }
                ContentTree::Leaf(leaf_type) => match leaf_type {
                    LeafType::Literal(mut tokens) => {
                        if data.to_uppercase() == data {
                            tokens.push(token.into());
                            ContentTree::Leaf(LeafType::Literal(tokens))
                        } else {
                            ContentTree::Branch {
                                branch_type: BranchType::Concatenation(token.clone().into()),
                                head: Box::new(ContentTree::Leaf(LeafType::Core(token.into()))),
                                tail: Box::new(self.state.clone()),
                            }
                        }
                    }
                    LeafType::Core(_) => {
                        if data.to_uppercase() == data {
                            ContentTree::Branch {
                                branch_type: BranchType::Concatenation(token.clone().into()),
                                head: Box::new(ContentTree::Leaf(LeafType::Literal(vec![
                                    token.clone().into(),
                                ]))),
                                tail: Box::new(self.state.clone()),
                            }
                        } else {
                            ContentTree::Branch {
                                branch_type: BranchType::Concatenation(token.clone().into()),
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

    pub fn feed(&mut self, tokens: Vec<TokenWithLocation>) -> Result<(), ContentParseError> {
        for (pointer, token) in tokens.clone().iter().enumerate() {
            if let Err(e) = self.feed_one(token.clone()) {
                return Err(ContentParseError::ErrorInString {
                    remainder: tokens[pointer..].to_vec(),
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

    pub fn get_current_state(&self) -> &ContentTree {
        &self.state
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
}
