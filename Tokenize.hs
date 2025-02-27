module Tokenize where

import ValidWords (conceptMap, FeaturedWord(..))
import qualified Data.Map as Map

data Token =
    PlainToken String Int Int
    | PunctuationToken String Int Int
    deriving (Show, Eq)

data TokenizationState = TokenizationState {
    buffer :: String,
    tokens :: [Token],
    parsedChars :: Int,
    line :: Int,
    column :: Int
} deriving (Show, Eq)

--tokenize :: String -> [Token]

commitFromPunctuation :: TokenizationState -> [ Token ] -> TokenizationState
commitFromPunctuation (TokenizationState buffer tokens u l o) extraTokens =
    TokenizationState "" (tokens ++ (if null buffer then [] else [PlainToken buffer l o]) ++ extraTokens) (u + 1) l (o + 1)

commitFromSpace :: TokenizationState -> TokenizationState
commitFromSpace (TokenizationState buffer tokens u l o) 
    | null buffer = TokenizationState "" tokens (u + 1) l (o + 1)
    | otherwise = TokenizationState "" (tokens ++ [PlainToken buffer l (o)]) (u + 1) l (o + 1)

commitFromNewline :: TokenizationState -> TokenizationState
commitFromNewline (TokenizationState buffer tokens u l o) 
    = TokenizationState "" (tokens ++ [PunctuationToken "\n" l o]) (u + 1) (l + 1) 1

addCharacter :: TokenizationState -> Char -> TokenizationState
addCharacter (TokenizationState buffer tokens u l o) c 
    = TokenizationState (buffer ++ [c]) tokens (u + 1) l (o + 1)

tokenizeInto :: TokenizationState -> Char -> TokenizationState
tokenizeInto state@(TokenizationState buffer tokens u l o) c
    | c `elem` ['.', ':', ','] 
        = commitFromPunctuation state [ PunctuationToken [c] l o ]
    | c `elem` [' ', '\t'] = commitFromSpace state
    | c == '\n' = 
        if null buffer
        then commitFromNewline state
        else error ("Unexpected newline character at " ++ show u)
    | otherwise = addCharacter state c

tokenizeStringInto :: TokenizationState -> String -> TokenizationState
tokenizeStringInto state str = foldl tokenizeInto state str

grabFeaturedWord :: String -> Int -> Int -> FeaturedWord
grabFeaturedWord word l o = 
    case Map.lookup word conceptMap of
        Just featuredWord -> featuredWord
        Nothing -> error ("Unknown word: >" ++ word ++ "< at line " ++ show l ++ " at offset " ++ show o)

emitAffixes :: Token -> [Token]
emitAffixes (PlainToken value l o) = 
    let (FeaturedWord word prefix suffix sufsuffix) = grabFeaturedWord value l o
        prefixLength = maybe 0 length prefix
        suffixLength = maybe 0 length suffix
        sufsuffixLength = maybe 0 length sufsuffix
        wordLength = length word
        
        -- Calculate start position (offset is the end position)
        startPos = o - length value
        
        -- Calculate positions for each component
        prefixPos = startPos
        wordPos = prefixPos + prefixLength
        suffixPos = wordPos + wordLength
        sufsuffixPos = suffixPos + suffixLength
        
        -- Create tokens for each component if they exist
        prefixToken = maybe [] (\x -> [ PlainToken x l prefixPos ]) prefix
        wordToken = [ PlainToken word l wordPos ]
        suffixToken = maybe [] (\x -> [ PlainToken x l suffixPos ]) suffix
        sufsuffixToken = maybe [] (\x -> [ PlainToken x l sufsuffixPos ]) sufsuffix
    in prefixToken ++ wordToken ++ suffixToken ++ sufsuffixToken

emitAffixes token = [ token ]

withAffixes :: [Token] -> [Token]
withAffixes tokens = concatMap emitAffixes tokens

validate :: [Token] -> [Token]
validate tokens = map check tokens
    where 
        check (PlainToken "" l o) = error ("Empty plain token from line " ++ show l ++ " at offset " ++ show o)
        check (PunctuationToken "" l o) = error ("Empty punctuation token from line" ++ show l ++ " at offset " ++ show o)
        check token = token

tokenizeString :: String -> [Token]
tokenizeString str = validate $ withAffixes $ tokens $ tokenizeStringInto (TokenizationState "" [] 0 1 1) str

