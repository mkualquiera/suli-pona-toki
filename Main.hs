module Main where

import System.Environment (getArgs)
import System.IO (readFile)
import Tokenize (tokenizeString, Token(..))
import ValidWords (FeaturedWord(..))

-- Define ANSI color codes directly
blueColor :: String
blueColor = "\ESC[94m"

magentaColor :: String
magentaColor = "\ESC[95m"

greenColor :: String
greenColor = "\ESC[92m"

cyanColor :: String
cyanColor = "\ESC[96m"

redColor :: String
redColor = "\ESC[91m"

resetCode :: String
resetCode = "\ESC[0m"

-- Define color functions
colorPlainToken :: IO ()
colorPlainToken = putStr blueColor

colorPrefixToken :: IO ()
colorPrefixToken = putStr magentaColor

colorSuffixToken :: IO ()
colorSuffixToken = putStr greenColor

colorSufsuffixToken :: IO ()
colorSufsuffixToken = putStr cyanColor

colorPunctuationToken :: IO ()
colorPunctuationToken = putStr redColor

resetColor :: IO ()
resetColor = putStr resetCode

-- Print token with appropriate color
printToken :: Token -> IO ()
printToken (PlainToken str line column) = do
    colorPlainToken
    putStr $ str ++ "(" ++ show line ++ "," ++ show column ++ ")"
    resetColor
    putStr " "
printToken (PunctuationToken str line column) = do
    colorPunctuationToken
    putStr $ str ++ "(" ++ show line ++ "," ++ show column ++ ")"
    resetColor
    putStr " "

main :: IO ()
main = do
    args <- getArgs
    case args of
        [filePath] -> do
            content <- readFile filePath
            let tokens = tokenizeString content
            mapM_ printToken tokens
            putStrLn ""
        _ -> putStrLn "Usage: ./tokenizeString <file-path>"