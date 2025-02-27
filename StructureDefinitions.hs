module StructureDefinitions where

-- | Basic token types in suli pona toki
data Token = 
    PlainToken String                     -- ^ A regular word (e.g., "mi", "sina")
  | SuffixToken String String             -- ^ A word with a grammatical suffix (e.g., "jansu" as ("jan", "su"))
  | PrefixToken String String             -- ^ A word with a prefix (e.g., "anpona" as ("an", "pona"))
  | PunctuationToken Char                 -- ^ Punctuation characters
  deriving (Show, Eq)

-- | A sentence part with its semantic tree representation
data SentencePart = SentencePart {
    tokens :: [Token],             -- ^ The tokens in this part
    tree :: Maybe Tree             -- ^ The semantic tree representation (using pi/no)
} deriving (Show, Eq)

-- | A semantic tree for representing pi/no relationships
data Tree = Leaf (Maybe String) | Node Tree Tree
  deriving (Show, Eq)

-- | A verb with its associated objects
data Verb = Verb {
    content :: String,             -- ^ The verb content (e.g., "tawa")
    objects :: [String]            -- ^ The objects associated with this verb
} deriving (Show, Eq)

-- | A sentence structure
data Sentence = Sentence {
    context :: Maybe SentencePart,  -- ^ Context part (with -la suffix)
    subject :: Maybe SentencePart,  -- ^ Subject part (with -su suffix)
    verbs :: [Verb],                -- ^ Verbs (with -pa suffix) and their objects
    endTokens :: [Token]            -- ^ Any remaining tokens at the end
} deriving (Show, Eq)

-- | A paragraph is a collection of sentences
data Paragraph = Paragraph [Sentence]
  deriving (Show, Eq)

-- | A document is a collection of paragraphs
data Document = Document [Paragraph]
  deriving (Show, Eq)

-- | Empty/default constructors
emptyDocument :: Document
emptyDocument = Document []

emptyParagraph :: Paragraph
emptyParagraph = Paragraph []

emptySentence :: Sentence
emptySentence = Sentence {
    context = Nothing,
    subject = Nothing,
    verbs = [],
    endTokens = []
}

emptySentencePart :: SentencePart
emptySentencePart = SentencePart {
    tokens = [],
    tree = Nothing
}

emptyVerb :: Verb
emptyVerb = Verb {
    content = "",
    objects = []
}

-- | Create a new Tree
emptyTree :: Tree
emptyTree = Leaf Nothing