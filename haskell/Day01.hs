{-# LANGUAGE BangPatterns #-}

module Main where

import AoC              (day)
import Data.List        (foldl', foldl1', sort)
import Data.List.Split  (splitWhen)
import Data.Word        (Word32)

{-# INLINE sum1 #-}
sum1 :: [Word32] -> Word32
sum1 = foldl1' (+)

{-# INLINE insertIntoTopList #-}
insertIntoTopList :: [Word32] -> Word32 -> [Word32]
insertIntoTopList tts@(!t : ts) !x = if t < x then go ts x else tts
  where
    go :: [Word32] -> Word32 -> [Word32]
    go  tts@(!t : ts) !x = if t < x then t : go ts x else x : tts
    go !tts           !x = x : tts

main :: IO ()
main = do
    input <- day 1
    let topThreeElves = foldl' insertIntoTopList [0, 0, 0]
                      . map (sum1 . map read)
                      . splitWhen null
                      . lines
                      $ input
    putStrLn . showString "Part 1: " . show . last $ topThreeElves
    putStrLn . showString "Part 2: " . show . sum1 $ topThreeElves
