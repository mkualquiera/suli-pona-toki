module Parse where

import Tokenize (Token(..), tokenizeString)
import Text.Printf (printf)
import Data.List (intercalate)

import Data.Typeable

data PrepositionType = Direction | Origin | Using | Location | Metacommentary
    deriving (Show, Eq)

data Preposition = Preposition {
    prepType :: PrepositionType,
    prepContent :: [ Token ],
    prepRemainder :: [ Token ]
} deriving (Show, Eq)

emptyPreposition :: PrepositionType -> Preposition
emptyPreposition prepType = Preposition {
    prepType = prepType,
    prepContent = [],
    prepRemainder = []
}

data Object = Object {
    objContent :: [ Token ],
    objRemainder :: [ Token ],
    objPrepositions :: [ Preposition ]
} deriving (Show, Eq, Typeable)

emptyObject :: Object
emptyObject = Object {
    objContent = [],
    objRemainder = [],
    objPrepositions = []
}

data Verb = Verb {
    verbContent :: [ Token ],
    objects :: [ Object ],
    verbRemainder :: [ Token ],
    verbPrepositions :: [ Preposition ]
} deriving (Show, Eq, Typeable)


emptyVerb :: Verb
emptyVerb = Verb {
    verbContent = [],
    objects = [],
    verbRemainder = [],
    verbPrepositions = []
}

data Subject = Subject {
    subjectContent :: [ Token ],
    subjectRemainder :: [ Token ],
    subjectPrepositions :: [ Preposition ]
} deriving (Show, Eq, Typeable)

emptySubject :: Subject = Subject {
    subjectContent = [],
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

type Paragraph = [ Sentence ]

type Document = [ Paragraph ]


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
        preposition{prepContent = prepRemainder preposition, 
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
        subjectContent = subjectRemainder subject,
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
            emptyObject{objContent=verbRemainder verb, 
                objPrepositions=(verbPrepositions verb)}
        ], 
        verbRemainder = [],
        verbPrepositions = []
    }
    
parseTokensIntoVerb verb (PlainToken "pa" _ _ : tokens) = 
    verb { verbContent = verbRemainder verb, verbRemainder = tokens }
    
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
    : _) = error ("Multiple subjects in sentence: " ++ show sentence)
    
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


parseTokens :: [Token] -> Sentence
parseTokens = parseTokensInto emptySentence

parseString :: String -> Sentence
parseString string = parseTokens $ tokenizeString string