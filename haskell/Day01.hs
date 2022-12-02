module Main where

import AoC              (day)
import Data.List        (foldl1', sortBy)
import Data.List.Split  (splitWhen)
import Data.Word        (Word32)

{-# INLINE sum1 #-}
sum1 :: [Word32] -> Word32
sum1 = foldl1' (+)

main :: IO ()
main = do
    input <- day 1
    let xs = sortBy (flip compare) . map (sum1 . map read) . splitWhen null . lines $ input
    putStrLn . showString "Part 1: " . show . head $ xs
    putStrLn . showString "Part 2: " . show . sum1 . take 3 $ xs
