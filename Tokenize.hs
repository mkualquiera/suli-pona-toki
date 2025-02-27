module Tokenize where

import ValidWords (conceptMap, FeaturedWord(..))
import qualified Data.Map as Map

data Token =
    PlainToken String
    | PunctuationToken String
    deriving (Show, Eq)

data TokenizationState = TokenizationState {
    buffer :: String,
    tokens :: [Token],
    parsedChars :: Int
} deriving (Show, Eq)

--tokenize :: String -> [Token]

commitFromPunctuation :: TokenizationState -> [ Token ] -> TokenizationState
commitFromPunctuation (TokenizationState buffer tokens u) extraTokens =
    TokenizationState "" (tokens ++ [PlainToken buffer] ++ extraTokens) (u + 1)

commitFromSpace :: TokenizationState -> TokenizationState
commitFromSpace (TokenizationState buffer tokens u) 
    | null buffer = TokenizationState "" tokens (u + 1)
    | otherwise = TokenizationState "" (tokens ++ [PlainToken buffer]) (u + 1)

addCharacter :: TokenizationState -> Char -> TokenizationState
addCharacter (TokenizationState buffer tokens u) c 
    = TokenizationState (buffer ++ [c]) tokens (u + 1)

tokenizeInto :: TokenizationState -> Char -> TokenizationState
tokenizeInto state@(TokenizationState buffer tokens u) c
    | c `elem` ['.', ':', ','] 
        = commitFromPunctuation state [ PunctuationToken [c] ]
    | c `elem` [' ', '\t'] = commitFromSpace state
    | c == '\n' = 
        if null buffer
        then commitFromSpace state
        else error ("Unexpected newline character at " ++ show u)
    | otherwise = addCharacter state c

tokenizeStringInto :: TokenizationState -> String -> TokenizationState
tokenizeStringInto state str = foldl tokenizeInto state str

grabFeaturedWord :: String -> FeaturedWord
grabFeaturedWord word = 
    case Map.lookup word conceptMap of
        Just featuredWord -> featuredWord
        Nothing -> error ("Unknown word: >" ++ word ++ "<")

emitAffixes :: Token -> [Token]
emitAffixes (PlainToken value) = 
    let (FeaturedWord word prefix suffix sufsuffix) = grabFeaturedWord value
        prefixToken = maybe [] (\x -> [ PlainToken x ]) prefix
        suffixToken = maybe [] (\x -> [ PlainToken x ]) suffix
        sufsuffixToken = maybe [] (\x -> [ PlainToken x ]) sufsuffix
    in prefixToken ++ [ PlainToken word ] ++ suffixToken ++ sufsuffixToken

emitAffixes token = [ token ]

withAffixes :: [Token] -> [Token]
withAffixes tokens = concatMap emitAffixes tokens

tokenizeString :: String -> [Token]
tokenizeString str = withAffixes $ tokens $ tokenizeStringInto (TokenizationState "" [] 0) str

