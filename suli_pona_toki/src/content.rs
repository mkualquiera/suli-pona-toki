use crate::tokens::{Token, TokenWithLocation};

#[derive(Debug, Clone)]
enum LeafType {
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
enum BranchType {
    Concatenation(Token),
    Property(Token),
    InnerGlue(Token),
    Union(Token),
    Alternative(Token),
}

#[derive(Debug, Clone)]
enum ContentTree {
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
}

#[derive(Debug)]
enum ContentParseError {
    NotPlainToken(TokenWithLocation),
    GlueWithoutBranch(TokenWithLocation),
}

impl ContentTree {
    fn parse_into(self, token: TokenWithLocation) -> Result<Self, ContentParseError> {
        let data = if let Token::Plain(data) = token.clone().into() {
            data
        } else {
            return Err(ContentParseError::NotPlainToken(token));
        };
        match data.as_str() {
            "un" => Ok(ContentTree::Branch {
                branch_type: BranchType::Union(token.into()),
                head: Box::new(ContentTree::Terminal),
                tail: Box::new(self),
            }),
            "nu" => Ok(ContentTree::Branch {
                branch_type: BranchType::Alternative(token.into()),
                head: Box::new(ContentTree::Terminal),
                tail: Box::new(self),
            }),
            "pi" => Ok(ContentTree::Branch {
                branch_type: BranchType::Property(token.into()),
                head: Box::new(ContentTree::Terminal),
                tail: Box::new(self),
            }),
            "no" => {
                if let ContentTree::Branch {
                    branch_type,
                    head,
                    tail,
                } = self
                {
                    Ok(ContentTree::Branch {
                        branch_type: BranchType::InnerGlue(token.into()),
                        head: Box::new(ContentTree::Branch {
                            branch_type,
                            head: Box::new(ContentTree::Terminal),
                            tail: head,
                        }),
                        tail,
                    })
                } else {
                    Err(ContentParseError::GlueWithoutBranch(token))
                }
            }
            _ => match self.clone() {
                ContentTree::Terminal => {
                    if data.to_uppercase() == data {
                        Ok(ContentTree::Leaf(LeafType::Literal(vec![token.into()])))
                    } else {
                        Ok(ContentTree::Leaf(LeafType::Core(token.into())))
                    }
                }
                ContentTree::Leaf(leaf_type) => match leaf_type {
                    LeafType::Literal(mut tokens) => {
                        if data.to_uppercase() == data {
                            tokens.push(token.into());
                            Ok(ContentTree::Leaf(LeafType::Literal(tokens)))
                        } else {
                            Ok(ContentTree::Branch {
                                branch_type: BranchType::Concatenation(token.clone().into()),
                                head: Box::new(ContentTree::Leaf(LeafType::Core(token.into()))),
                                tail: Box::new(self),
                            })
                        }
                    }
                    LeafType::Core(_) => {
                        if data.to_uppercase() == data {
                            Ok(ContentTree::Branch {
                                branch_type: BranchType::Concatenation(token.clone().into()),
                                head: Box::new(ContentTree::Leaf(LeafType::Literal(vec![
                                    token.clone().into(),
                                ]))),
                                tail: Box::new(self),
                            })
                        } else {
                            Ok(ContentTree::Branch {
                                branch_type: BranchType::Concatenation(token.clone().into()),
                                head: Box::new(ContentTree::Leaf(LeafType::Core(token.into()))),
                                tail: Box::new(self),
                            })
                        }
                    }
                },
                ContentTree::Branch {
                    branch_type,
                    head,
                    tail,
                } => Ok(ContentTree::Branch {
                    branch_type,
                    head: Box::new(head.parse_into(token)?),
                    tail,
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::TokenizationState;

    use super::*;

    #[test]
    fn test_easy() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("sina suli mun luka linja ").unwrap();
        let mut tree = ContentTree::Terminal;
        for token in tokens {
            tree = tree.parse_into(token).unwrap();
        }
        insta::assert_snapshot!(tree.as_natural());
    }

    #[test]
    fn test_medium() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed("sina pona jan pi sitelen ilo ").unwrap();
        let mut tree = ContentTree::Terminal;
        for token in tokens {
            tree = tree.parse_into(token).unwrap();
        }
        insta::assert_snapshot!(tree.as_natural());
    }

    #[test]
    fn test_hard() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer
            .feed("nasa telo pi ike jan no suli tomo ")
            .unwrap();
        let mut tree = ContentTree::Terminal;
        for token in tokens {
            tree = tree.parse_into(token).unwrap();
        }
        insta::assert_snapshot!(tree.as_natural());
    }

    #[test]
    fn test_hardest() {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer
            .feed("nasa telo nu loje telo pi PA PU TO JAN no ike jan no suli tomo un tawa tomo ")
            .unwrap();
        let mut tree = ContentTree::Terminal;
        for token in tokens {
            tree = tree.parse_into(token).unwrap();
        }
        insta::assert_snapshot!(tree.as_natural());
    }
}
