-- Define data types for Tree structure
data Tree = Leaf (Maybe String) | Node Tree Tree
  deriving (Show, Eq)

-- Root definition
root :: Tree
root = Leaf Nothing

-- Helper function to prune the tree
prune :: Tree -> Tree
prune (Node head (Leaf Nothing)) = head
prune tree = tree

-- Main parsing function
parseInto :: Tree -> String -> Tree
parseInto state token = prune $ parseIntoCore state token

-- Core parsing logic
parseIntoCore :: Tree -> String -> Tree
parseIntoCore state "pi"             = Node (Leaf Nothing) state
parseIntoCore (Node head tail) "no"  = Node (Node (Leaf Nothing) head) tail
parseIntoCore (Leaf value) token     = Node (Leaf (Just token)) (Leaf value)
parseIntoCore (Node head tail) token = Node (parseInto head token) tail

-- Parse a list of tokens
parseTokens :: [String] -> Tree
parseTokens = foldl parseInto root

-- Parse a space-separated string
parseString :: String -> Tree
parseString str = parseTokens (words str)


-- Map a tree to its natural language representation
toNatural :: Tree -> String
toNatural (Leaf (Just value)) = value
toNatural (Node head tail) = "(" ++ toNatural tail ++ " " ++ toNatural head ++ ")"
toNatural (Leaf Nothing) = ""

-- Example usage:
-- > toNatural $ parseString "nasa telo pi ike jan no suli tomo"
-- "((nasa telo) ((jan ike) (suli tomo)))"