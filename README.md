# suli pona toki: A Comprehensive Guide

## Introduction

suli pona toki is a dialect of toki pona inspired by Japanese sentence structure and Forth-like stack-based processing. It introduces several significant structural changes while maintaining toki pona's vocabulary and philosophical simplicity:

1. **Particle-Based Grammar**: All grammatical functions are expressed through suffix particles
2. **Head-Final Structure**: Modifiers precede the words they modify (opposite of toki pona)
3. **The "no" Connector**: Creates possesive or attribute-holding grouping boundaries (similar to Japanese "の")
4. **The "an-" Prefix**: Systematically creates opposites for any concept
5. **The "lu-" Particle**: Provides a pragmatic layer for expressing the speaker's perspective. The literal meaning of the sentence stays the same, but the listener can infer the speaker's attitude or perspective on the information.
6. **Subject-Object-Verb structure**: The verb is now the last element in the sentence. This allows more easily talking about multiple objects in a single sentence.

Together, these changes create a more flexible language capable of greater precision and complexity while maintaining the core simplicity of toki pona's vocabulary.

## Core Philosophy

- All content words can serve as nouns, verbs, or modifiers in any context. This avoids the need for a large vocabulary, and gives new uses to previously limited words. This is true even of words like "kepeken", which can only be a preposition in toki pona.
- Grammatical functions are exclusively handled by suffix particles. This allows for a more flexible and expressive system that avoids ambiguity. For example the word "lon" can freely become a content word, without having to also be a preposition. 
- Modifiers come before the modified word (head-final). This is to be consistent with the particle-based grammar.
- Opposites can be systematically derived through the "an-" prefix. This allows for a more systematic and predictable way to express antonyms, making previous (now presumably obsolete) words like "ike" be mainly useful for specific nuance.
- More complex concepts can be built using a combination of `pi` and `no` to create nested relationships. This allows for more precise expression of complex ideas.

## Phonology

suli pona toki maintains toki pona's phonology:
- Consonants: p, t, k, s, m, n, l, j, w
- Vowels: a, e, i, o, u
- Syllable structure: (C)V(n)

Particles can be pronounced as separate syllables or as part of the word they attach to. For example, "jansu" can be pronounced as "jan-su" or "jansu".

## Grammatical Particles

Particles in suli pona toki are attached as suffixes to their associated content words:

| Suffix | Function | Original Word | Example |
|--------|----------|---------------|---------|
| -su | Subject marker | suli (important) | jansu = "person (as subject)" |
| -pa | Verb/action marker | pali (do/work) | tawapa = "go (as action)" |
| -jo | Object marker | jo (have) | tomojo = "house (as object)" |
| -ta | To/toward | tawa (to/toward) | tomota = "to the house" |
| -na | From/because | tan (from) | tomona = "from the house" |
| -ki | Using/with | kepeken (use) | iloki = "with a tool" |
| -lo | At/in/on | lon (at) | tomolo = "at the house" |
| -la | Context marker | la (if/when) | monsi tenpola = "before..." |
| -en | And | en (and) | mije jan en meli jan = "the man and the woman" |
| -nu | Or/alternative | anu (or) | minu sina = "you or I" |
| -lu | Discourse perspective marker; transforms phrases into adverbials or meta-commentary | lukin (to see) | taso lonlu = "but of course...", mi lonlu = "honestly", suli ponalu = "incredibly enough", kin nasinlu = "by the way"
 |

### The "-lu" Particle

The "-lu" suffix transforms phrases into discourse markers and adverbials that express the speaker's perspective or attitude toward what's being said. It creates meta-commentary without requiring additional vocabulary.

Types of "-lu" Expressions:

1. Discourse connectors:

taso lonlu = "but of course..."
ante nasinlu = "anyway..."
taso kenlu = "but also..."

2. Speaker attitude markers:

mi lonlu = "honestly" (lit: "from my true perspective")
suli ponalu = "incredibly enough" (lit: "from a greatly good perspective")
kin nasinlu = "by the way" (lit: "also from this path perspective")

3. Inclusivity/exclusivity markers:

mi kinlu = "me too/as well" (lit: "me also-perspective")
sinasu antasolu ponapa = "she's not just beautiful" (lit: "she (not)only-perspective beautiful")

This particle allows speakers to signal how they're approaching a topic, creating a pragmatic layer that makes conversation more natural and nuanced.

## The "an-" Prefix

The "an-" prefix can be attached to any content word to express its conceptual opposite:

| Original | With "an-" | Standard toki pona Equivalent |
|----------|------------|------------------------------|
| pona (good) | anpona | ike (bad) |
| suli (big) | ansuli | lili (small) |
| moli (death) | anmoli | lon (existence) |
| open (open) | anopen | pini (close/end) |
| wawa (strong) | anwawa | wawa ala (not strong) |
| suno (light) | ansuno | pimeja (darkness) |
| awen (stay) | anawen | kama (become/arrive) |

Unlike simple negation with "ala", the "an-" prefix creates a true opposite concept. For example, "anpona" (not-good) is distinct from "ike" (bad).

## Word Order

### Basic Sentence Structure

```
[[context]-la]? [ [prepositions]? [[modifiers]? [subject]]-su ]? [ [prepositions]? [[modifiers] [verb]]-pa ]? [ [prepositions]? [[modifiers] [object]]-jo ]?
```

### Prepositional Phrases

```
[context]-la
[location]-lo
[destination]-ta
[source]-na
[instrument]-ki
[alternative]-nu
[framing]-lu
```

These prepositions can be placed in different positions within the sentence to create more natural expressions:

Examples:
- ``tomolo jansu tawapa lekojo`` = "The person who is at the house moves the box."
- ``jansu tomolo tawapa lekojo`` = "The person moves at the house the box."
- ``jansu tawapa tomolo lekojo`` = "The person moves the box at the house."

Notice how all of these sentences are effectively the same, but the word order changes the emphasis.

## Modifier Structure

### Simple modification

Modifiers precede the words they modify:

- `nasa telo tawa tomo` = "crazy water moving structure" = a moving structure of water, and this structure happens to be crazy. Like a car that is melting in a crazy way.

### Modification with pi

The "pi" particle creates a new semantic unit, where what follows it becomes the "core" concept (even multiple words), and what precedes it becomes a tail:

- `nasa telo pi tawa tomo` = "(crazy water) (moving structure)" = a moving structure of a crazy water. Like a limousine where they serve alcohol.
- `nasa telo tawa pi tomo` = "(crazy water moving) structure" = a structure with that moves in a crazy watery way. Like some sort of sci-fi alien contraption.

### Advanced Modification with "no"

The "no" particle can be thought of as turning the current head into the nuance of a new head. This allows for complex semantic grouping:

- `nasa telo pi ike jan pi suli tomo` = "((crazy water) (bad person)) (big structure)" = The mansion owned by a villain who is into making alcohol.
- `nasa telo pi ike jan no suli tomo` = "(crazy water) ((bad person) (big structure))" = The big bar owned by the villain.

Although intuitive, this grouping can be complex and requires practice to master. A reference haskell implementation for the precise parsing logic is provided in the appendix.

# Appendix

## Haskell Implementation of "no" and "pi" Parsing

```haskell
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
```

