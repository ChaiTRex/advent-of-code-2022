module Main where

import AoC                 (day)
import Data.Array.Unboxed  (UArray, (!), listArray)
import Data.Char           (ord)
import Data.List           (foldl1')
import Data.Word           (Word8, Word16)

{-# INLINE sum1 #-}
sum1 :: [Word16] -> Word16
sum1 = foldl1' (+)

{-# NOINLINE partOneScores #-}
partOneScores :: UArray Int Word8
partOneScores = listArray (0, 8) [4, 8, 3, 1, 5, 9, 7, 2, 6]

{-# NOINLINE partTwoScores #-}
partTwoScores :: UArray Int Word8
partTwoScores = listArray (0, 8) [3, 4, 8, 1, 5, 9, 2, 6, 7]

main :: IO ()
main = do
    input <- day 2
    let xs = map (\ (opponent : _ : me : _) -> 3*ord opponent + ord me - 283) . lines $ input
    putStrLn . showString "Part 1: " . show . sum1 . map (fromIntegral . (partOneScores !)) $ xs
    putStrLn . showString "Part 2: " . show . sum1 . map (fromIntegral . (partTwoScores !)) $ xs
