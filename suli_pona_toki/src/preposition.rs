use crate::{
    content::{ContentParseError, ContentTree, ContentTreeParser},
    tokens::{Token, TokenWithLocation},
};

#[derive(Debug, Clone)]
enum PrepositionType {
    Direction,
    Origin,
    Using,
    Location,
    Metacommentary,
    Manner,
}

impl PrepositionType {
    fn from_token(token: &Token) -> Option<Self> {
        match token.content() {
            "ta" => Some(PrepositionType::Direction),
            "na" => Some(PrepositionType::Origin),
            "ki" => Some(PrepositionType::Using),
            "lo" => Some(PrepositionType::Location),
            "lu" => Some(PrepositionType::Metacommentary),
            "sa" => Some(PrepositionType::Manner),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Preposition {
    preposition_type: PrepositionType,
    content: ContentTree,
}

pub struct PrepositionParser {
    remainder: ContentTreeParser,
}

impl PrepositionParser {
    pub fn new() -> Self {
        Self {
            remainder: ContentTreeParser::new(),
        }
    }

    pub fn close(self) -> Result<ContentTree, ContentParseError> {
        self.remainder.close()
    }

    pub fn feed_one(
        &mut self,
        token: TokenWithLocation,
    ) -> Result<Option<Preposition>, ContentParseError> {
        if let Some(preposition_type) = PrepositionType::from_token(token.peek_inner()) {
            let content = self.remainder.take()?;
            Ok(Some(Preposition {
                preposition_type,
                content,
            }))
        } else {
            self.remainder.feed_one(token)?;
            Ok(None)
        }
    }

    pub fn take(&mut self) -> Result<ContentTree, ContentParseError> {
        self.remainder.take()
    }

    pub fn is_empty(&self) -> Result<(), ContentParseError> {
        self.remainder.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenizationState;

    fn parse_prepositions(input: &str) -> Result<Vec<PrepositionType>, ContentParseError> {
        let mut tokenizer = TokenizationState::new();
        let tokens = tokenizer.feed(input).unwrap();
        let mut parser = PrepositionParser::new();

        let mut prepositions = Vec::new();
        for token in tokens {
            if let Some(prep) = parser.feed_one(token)? {
                prepositions.push(prep.preposition_type);
            }
        }

        parser.close()?;
        Ok(prepositions)
    }

    #[test]
    fn test_no_prepositions() {
        let result = parse_prepositions("sina pona jan suli ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_single_preposition() {
        let result = parse_prepositions("sina suli ta tomo ");
        assert!(result.is_ok());
        let prepositions = result.unwrap();
        assert_eq!(prepositions.len(), 1);
        assert!(matches!(prepositions[0], PrepositionType::Direction));
    }

    #[test]
    fn test_multiple_prepositions() {
        let result = parse_prepositions("sina pona ta tomo na jan ki ilo ");
        assert!(result.is_ok());
        let prepositions = result.unwrap();
        assert_eq!(prepositions.len(), 3);
        assert!(matches!(prepositions[0], PrepositionType::Direction));
        assert!(matches!(prepositions[1], PrepositionType::Origin));
        assert!(matches!(prepositions[2], PrepositionType::Using));
    }

    #[test]
    fn test_all_preposition_types() {
        let result = parse_prepositions("jan ta tomo na ilo ki moku lo supa lu nasa ");
        assert!(result.is_ok());
        let prepositions = result.unwrap();
        assert_eq!(prepositions.len(), 5);
        assert!(matches!(prepositions[0], PrepositionType::Direction));
        assert!(matches!(prepositions[1], PrepositionType::Origin));
        assert!(matches!(prepositions[2], PrepositionType::Using));
        assert!(matches!(prepositions[3], PrepositionType::Location));
        assert!(matches!(prepositions[4], PrepositionType::Metacommentary));
    }

    #[test]
    fn test_consecutive_prepositions() {
        let result = parse_prepositions("jan ta ma na ilo ki tomo ");
        assert!(result.is_ok());
        let prepositions = result.unwrap();
        assert_eq!(prepositions.len(), 3);
        assert!(matches!(prepositions[0], PrepositionType::Direction));
        assert!(matches!(prepositions[1], PrepositionType::Origin));
        assert!(matches!(prepositions[2], PrepositionType::Using));
    }

    #[test]
    fn test_invalid_content_structure() {
        let result = parse_prepositions("jan pi ta tomo ");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ContentParseError::InvalidTreeStructure(_)
        ));
    }

    #[test]
    fn test_incomplete_structure() {
        let result = parse_prepositions("jan suli pi ");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ContentParseError::InvalidTreeStructure(_)
        ));
    }
}
