{-# LANGUAGE UndecidableInstances #-}

module ToHTML where

import Parse
import Tokenize (Token(..))
import Data.Typeable

typeName :: Typeable a => a -> String
typeName x = show (typeOf x)

class ToHTML a where
    innerContent :: a -> String

class RenderHTML a where
    renderHTML :: a -> String


-- Default implementation for Typeable + ToHTML types
instance (Typeable a, ToHTML a) => RenderHTML a where
    renderHTML x = "<" ++ typeName x ++ ">" ++ innerContent x ++ "</" 
        ++ typeName x ++ ">"

instance {-# OVERLAPPING #-} RenderHTML ContentTree where
    renderHTML Ending = ""
    renderHTML (Leaf tok) = renderHTML tok
    renderHTML (ContentBranch { h=headVal, t=tailVal, branchType=Property _})
        = "<Property>" ++ renderHTML tailVal ++ "</Property>" ++ renderHTML headVal
    renderHTML (ContentBranch { h=headVal, t=tailVal, branchType=JoinInner _})
        = "<Join><JoinTail>" ++ renderHTML tailVal ++ "</JoinTail><JoinHead>" ++ renderHTML headVal ++ "</JoinHead></Join>"
    renderHTML (ContentBranch { h=headVal, t=tailVal })
        = renderHTML tailVal ++ " " ++ renderHTML headVal

-- Special override just for Sentence
instance {-# OVERLAPPING #-} RenderHTML Token where
    renderHTML (PlainToken t _ _) = t
    renderHTML (PunctuationToken t _ _) = t

instance {-# OVERLAPPING #-} RenderHTML a => RenderHTML [ a ] where
    renderHTML [] = ""
    renderHTML (x:xs) = renderHTML x ++ " " ++ renderHTML xs
    
instance {-# OVERLAPPING #-} RenderHTML Preposition where
    renderHTML (Preposition prepType prepContent prepRemainder) = 
        "<Preposition class=\"" ++ show prepType ++ "\">" 
        ++ renderHTML prepContent ++ "</Preposition>"

instance ToHTML Object where
    innerContent (Object objContent objRemainder objPrepositions) = 
        renderHTML objContent ++ renderHTML objPrepositions

instance ToHTML Verb where
    innerContent (Verb verbContent objects verbRemainder verbPrepositions) = 
        renderHTML verbPrepositions ++ renderHTML objects ++ renderHTML verbContent

instance ToHTML Subject where
    innerContent (Subject subjectContent subjectRemainder subjectPrepositions) = 
        renderHTML subjectContent ++ renderHTML subjectPrepositions

instance ToHTML Sentence where
    innerContent (Sentence context subject verb apposition sentenceRemainder) = 
        (case context of
            Nothing -> ""
            Just c -> "<Context>" ++ renderHTML c ++ "</Context>") ++
        (case subject of
            Nothing -> ""
            Just s -> renderHTML s) ++
        (case verb of
            Nothing -> ""
            Just v -> renderHTML v) ++
        (case apposition of
            Nothing -> ""
            Just a -> "<Apposition>" ++ renderHTML a ++ "</Apposition>") 

instance {-# OVERLAPPING #-} RenderHTML ParagraphElement where
    renderHTML (SentenceElement s) = renderHTML s
    renderHTML LineBreak = "<br />"

instance {-# OVERLAPPING #-} RenderHTML Paragraph where
    renderHTML (Paragraph {paragraphElements=p}) = "<p>" ++ (renderHTML p) ++ "</p>"

instance ToHTML Document where
    innerContent (Document {paragraphs=ps}) = renderHTML ps