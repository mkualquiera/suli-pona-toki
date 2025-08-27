use crate::content::{ContentParseError, ContentTree, ContentTreeParser};

enum PrepositionType {
    Direction,
    Origin,
    Using,
    Location,
    Metacommentary,
}

struct Preposition {
    preposition_type: PrepositionType,
    content: ContentTree,
}

struct PrepositionParser {
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
}
