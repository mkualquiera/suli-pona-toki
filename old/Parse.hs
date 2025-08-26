module Parse where

import Tokenize (Token(..), tokenizeString)
import Text.Printf (printf)
import Data.List (intercalate)

import Data.Typeable

data ContentBranchingType = Concatenation Token | Property Token | JoinInner Token 
    | JoinOuter Token | Union Token | Alternative Token deriving (Show, Eq)

data ContentTree = Ending | Leaf Token | ContentBranch {
    branchType :: ContentBranchingType,
    h :: ContentTree,
    t :: ContentTree
} deriving (Show, Eq)

parseIntoTree :: ContentTree -> Token -> ContentTree
parseIntoTree state token@(PlainToken "un" _ _) = ContentBranch {
    branchType = Union token,
    h = Ending,
    t = state
}
parseIntoTree state token@(PlainToken "nu" _ _) = ContentBranch {
    branchType = Alternative token,
    h = Ending,
    t = state
}
parseIntoTree state token@(PlainToken "pi" _ _) = ContentBranch {
    branchType = Property token,
    h = Ending,
    t = state
}
parseIntoTree state@(ContentBranch {h=headVal, t=tailVal, branchType=bt}) 
    token@(PlainToken "no" _ _) = ContentBranch {
        branchType = bt,
        h = ContentBranch {
            branchType = JoinInner token,
            h = Ending,
            t = headVal
        },
        t = tailVal
    }
parseIntoTree state@(Leaf value) token = ContentBranch {
    branchType = Concatenation token,
    h = Leaf token,
    t = Leaf value
}
parseIntoTree Ending token = Leaf token
parseIntoTree state@(ContentBranch {h=headVal, t=tailVal}) token = 
    state {
        h = parseIntoTree headVal token,
        t = tailVal
    }

parseTree :: [Token] -> ContentTree
parseTree = foldl parseIntoTree Ending

data PrepositionType = Direction | Origin | Using | Location | Metacommentary
    deriving (Show, Eq)

data Preposition = Preposition {
    prepType :: PrepositionType,
    prepContent :: ContentTree,
    prepRemainder :: [ Token ]
} deriving (Show, Eq)

emptyPreposition :: PrepositionType -> Preposition
emptyPreposition prepType = Preposition {
    prepType = prepType,
    prepContent = Ending,
    prepRemainder = []
}

data Object = Object {
    objContent :: ContentTree,
    objRemainder :: [ Token ],
    objPrepositions :: [ Preposition ]
} deriving (Show, Eq, Typeable)

emptyObject :: Object
emptyObject = Object {
    objContent = Ending,
    objRemainder = [],
    objPrepositions = []
}

data Verb = Verb {
    verbContent :: ContentTree,
    objects :: [ Object ],
    verbRemainder :: [ Token ],
    verbPrepositions :: [ Preposition ]
} deriving (Show, Eq, Typeable)


emptyVerb :: Verb
emptyVerb = Verb {
    verbContent = Ending,
    objects = [],
    verbRemainder = [],
    verbPrepositions = []
}

data Subject = Subject {
    subjectContent :: ContentTree,
    subjectRemainder :: [ Token ],
    subjectPrepositions :: [ Preposition ]
} deriving (Show, Eq, Typeable)

emptySubject :: Subject = Subject {
    subjectContent = Ending,
    subjectRemainder = [],
    subjectPrepositions = []
}                

data Sentence = Sentence {
    context :: Maybe Sentence,
    subject :: Maybe Subject,
    verbs :: Maybe [Verb],
    apposition :: Maybe Sentence,
    remainder :: [ Token ]
} deriving (Show, Eq, Typeable)


emptySentence :: Sentence
emptySentence = Sentence {
    context = Nothing,
    subject = Nothing,
    verbs = Nothing,
    apposition = Nothing,
    remainder = []
} 

data ParagraphElement = SentenceElement Sentence | LineBreak deriving (Show)

data Paragraph = Paragraph {
    paragraphElements :: [ ParagraphElement ],
    paragraphRemainder :: [ Token ]
} deriving (Show)

emptyParagraph :: Paragraph
emptyParagraph = Paragraph {
    paragraphElements = [],
    paragraphRemainder = []
}

data Document = Document {
    paragraphs :: [ Paragraph ],
    documentRemainder :: [ Token ]
} deriving (Show)

emptyDocument :: Document
emptyDocument = Document {
    paragraphs = [],
    documentRemainder = []
}

tokenToPrepositionType :: Token -> Maybe PrepositionType
tokenToPrepositionType (PlainToken "ta" _ _) = Just Direction
tokenToPrepositionType (PlainToken "na" _ _) = Just Origin
tokenToPrepositionType (PlainToken "ki" _ _) = Just Using
tokenToPrepositionType (PlainToken "lo" _ _) = Just Location
tokenToPrepositionType (PlainToken "lu" _ _) = Just Metacommentary
tokenToPrepositionType _ = Nothing

parseTokenIntoPreposition :: Preposition -> Token -> Preposition
parseTokenIntoPreposition preposition token = 
    preposition { prepRemainder = prepRemainder preposition ++ [token] }

parseTokensIntoPreposition :: Preposition -> [Token] -> Preposition
parseTokensIntoPreposition preposition [] = preposition
parseTokensIntoPreposition preposition (token : tokens) 
    | Just pT <- prepType = 
        preposition{prepContent = parseTree $ prepRemainder preposition, 
                    prepRemainder = tokens}
    | otherwise 
        = parseTokensIntoPreposition (parseTokenIntoPreposition 
            preposition token) tokens
  where
    prepType = tokenToPrepositionType token

parseTokenIntoSubject :: Subject -> Token -> Subject
parseTokenIntoSubject subject token = 
    subject { subjectRemainder = subjectRemainder subject ++ [token] }

parseTokensIntoSubject :: Subject -> [Token] -> Subject
parseTokensIntoSubject subject [] = subject
parseTokensIntoSubject subject (PlainToken "su" _ _ : tokens) = 
    parseTokensIntoSubject subject' tokens
  where
    subject' = subject {
        subjectContent = parseTree $ subjectRemainder subject,
        subjectRemainder = []
    }

parseTokensIntoSubject subject (token : tokens) 
    | Just pT <- prepType = 
        parseTokensIntoSubject (subject' pT) tokens
    | otherwise = parseTokensIntoSubject (parseTokenIntoSubject subject token) tokens
  where
    prepType = tokenToPrepositionType token
    subject' pT = subject {
        subjectPrepositions = subjectPrepositions subject ++ [
            parseTokensIntoPreposition (emptyPreposition pT) (
                subjectRemainder subject ++ [token]
            )
        ],
        subjectRemainder = []
    }

-- Refactored parsing functions with improved readability

-- Parse a single token into a verb
parseTokenIntoVerb :: Verb -> Token -> Verb
parseTokenIntoVerb verb token = 
    verb { verbRemainder = verbRemainder verb ++ [token] }

-- Parse a list of tokens into a verb
parseTokensIntoVerb :: Verb -> [Token] -> Verb
parseTokensIntoVerb verb [] = verb
parseTokensIntoVerb verb (PlainToken "mo" _ _ : tokens) = 
    parseTokensIntoVerb verb' tokens
  where
    verb' = verb { 
        objects = objects verb ++ [
            emptyObject{objContent=parseTree $ verbRemainder verb, 
                objPrepositions=(verbPrepositions verb)}
        ], 
        verbRemainder = [],
        verbPrepositions = []
    }
    
parseTokensIntoVerb verb (PlainToken "pa" _ _ : tokens) = 
    verb { verbContent = parseTree $ verbRemainder verb, verbRemainder = tokens }
    
parseTokensIntoVerb verb (token : tokens) 
    | Just pT <- prepType = 
        parseTokensIntoVerb (verb' pT) tokens
    | otherwise = parseTokensIntoVerb (parseTokenIntoVerb verb token) tokens
  where 
    prepType = tokenToPrepositionType token
    verb' pT = verb {
        verbPrepositions = verbPrepositions verb ++ [
            parseTokensIntoPreposition (emptyPreposition pT) (
                verbRemainder verb ++ [token]
            )
        ],
        verbRemainder = []
    }

-- Parse a single token into a sentence
parseTokenInto :: Sentence -> Token -> Sentence
parseTokenInto sentence token = 
    sentence { remainder = remainder sentence ++ [token] }

-- Parse a list of tokens into a sentence
parseTokensInto :: Sentence -> [Token] -> Sentence
parseTokensInto sentence [] = sentence
parseTokensInto sentence@Sentence{apposition=Just appo} tokens = 
    sentence { apposition = Just (parseTokensInto appo tokens) }

parseTokensInto sentence@Sentence{subject=Nothing} (token@(PlainToken "su" _ _) 
    : tokens) = parseTokensInto sentence' tokens
  where
    sentence' = sentence {
        subject = Just (parseTokensIntoSubject emptySubject (
            remainder sentence ++ [token])),
        remainder = []
    }

parseTokensInto sentence@Sentence{subject=Just _} (token@(PlainToken "su" _ _) 
    : _) = error ("Syntax error: multiple subjects in sentence: " ++ show sentence)
    
parseTokensInto sentence (token@(PlainToken "pa" _ _) : tokens) = 
    parseTokensInto sentence' tokens
  where
    parsedVerb = parseTokensIntoVerb emptyVerb (remainder sentence ++ [token])
    existingVerbs = maybe [] id (verbs sentence)
    sentence' = sentence {
        verbs = Just (existingVerbs ++ [parsedVerb]),
        remainder = []
    }

parseTokensInto sentence (token@(PlainToken "la" _ _) : tokens) = 
    parseTokensInto emptySentence {context = Just sentence} tokens

parseTokensInto sentence (token@(PunctuationToken ":" _ _) : tokens) =
    parseTokensInto (sentence { apposition = Just emptySentence }) tokens
    
parseTokensInto sentence (token : tokens) = 
    parseTokensInto (parseTokenInto sentence token) tokens


maybeLast :: [a] -> Maybe a
maybeLast [] = Nothing
maybeLast [x] = Just x
maybeLast (_:xs) = maybeLast xs

parseTokensIntoParagraph :: Paragraph -> [Token] -> Paragraph

parseTokensIntoParagraph paragraph@(Paragraph {paragraphElements = []}) 
    (PunctuationToken "\n" _ _ : tokens) = error ("Unexpected newline character")
parseTokensIntoParagraph paragraph (PunctuationToken "\n" _ _ : tokens) 
    -- error if the last element is a line break
    | LineBreak <- lastElement = error ("Syntax error: two line breaks in a row")
    | otherwise = parseTokensIntoParagraph paragraph' tokens
    where
        paragraph' = paragraph {
            paragraphElements = (paragraphElements paragraph) ++ [LineBreak]
        }
        lastElement = last $ paragraphElements paragraph

parseTokensIntoParagraph paragraph@(Paragraph {paragraphRemainder = []}) 
    (PunctuationToken "." _ _ : tokens) = error ("Unexpected period character")
parseTokensIntoParagraph paragraph (PunctuationToken "." _ _ : tokens)
    -- error if the last element is a line break and the remainder is empty
    | Just LineBreak <- lastElement, null (paragraphRemainder paragraph) 
        = error ("Syntax error: unexpected period character")
    | otherwise = parseTokensIntoParagraph paragraph' tokens
    where
        paragraph' = paragraph {
            paragraphElements = (paragraphElements paragraph) ++ [newSentence],
            paragraphRemainder = []
        }
        newSentence = SentenceElement $ parseTokensInto emptySentence 
            (paragraphRemainder paragraph)
        lastElement = maybeLast $ paragraphElements paragraph

parseTokensIntoParagraph paragraph (token : tokens) =
    parseTokensIntoParagraph paragraph' tokens
    where
        paragraph' = paragraph {
            paragraphRemainder = paragraphRemainder paragraph ++ [token]
        }

parseTokensIntoParagraph paragraph [] = paragraph

commitParagraph :: Document -> Document
commitParagraph Document{documentRemainder=[]} 
    = error ("Syntax error: tried to commit empty paragraph")
commitParagraph document = document {
    paragraphs = paragraphs document ++ [trimmedParagraph],
    documentRemainder = []
}
  where
    newParagraph = parseTokensIntoParagraph emptyParagraph (documentRemainder document)
    trimmedParagraph
        | LineBreak <- lastElement = newParagraph { 
                paragraphElements = init $ paragraphElements newParagraph 
            }
        | otherwise = newParagraph  
    lastElement = last $ paragraphElements newParagraph

-- Close document is like commitParagraph but it doesn't care if the remainder is empty
closeDocument :: Document -> Document
closeDocument document 
    | null (documentRemainder document) = document
    | otherwise = commitParagraph document

parseTokensIntoDocument :: Document -> [Token] -> Document
parseTokensIntoDocument document [] = document
parseTokensIntoDocument document@(Document{paragraphs=[], documentRemainder=[]}) 
    (PunctuationToken "\n" _ _ : tokens) = error ("Unexpected newline character")

parseTokensIntoDocument document (tok@(PunctuationToken "\n" _ _) : tokens) 
    | Just (PunctuationToken "\n" _ _) <- lastToken = 
        parseTokensIntoDocument (commitParagraph document) tokens
    | otherwise = parseTokensIntoDocument document' tokens
  where 
    lastToken = maybeLast (documentRemainder document)
    document' = document {
        documentRemainder = (documentRemainder document) ++ [tok]
    }

parseTokensIntoDocument document (tok : tokens) =
    parseTokensIntoDocument document' tokens
    where
        document' = document {
            documentRemainder = documentRemainder document ++ [tok]
        }
 
--parseTokens :: [Token] -> Sentence
--parseTokens = parseTokensInto emptySentence

-- parseTokens :: [Token] -> Paragraph
-- parseTokens = parseTokensIntoParagraph emptyParagraph

parseTokens :: [Token] -> Document
parseTokens = parseTokensIntoDocument emptyDocument

--parseString :: String -> Sentence
--parseString string = parseTokens $ tokenizeString string

parseString :: String -> Document
parseString string = closeDocument $ parseTokens $ tokenizeString string