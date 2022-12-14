module AoC where

import Data.Array        (Array, (!), listArray)
import Data.Word         (Word8)
import System.IO         (readFile)
import System.IO.Unsafe  (unsafePerformIO)

{-# INLINE day #-}
day :: Word8 -> IO String
day d = readFile (filepaths ! d)
  where
    filepaths :: Array Word8 String
    filepaths = listArray (1, 25) [
        "../day01.txt",
        "../day02.txt",
        "../day03.txt",
        "../day04.txt",
        "../day05.txt",
        "../day06.txt",
        "../day07.txt",
        "../day08.txt",
        "../day09.txt",
        "../day10.txt",
        "../day11.txt",
        "../day12.txt",
        "../day13.txt",
        "../day14.txt",
        "../day15.txt",
        "../day16.txt",
        "../day17.txt",
        "../day18.txt",
        "../day19.txt",
        "../day20.txt",
        "../day21.txt",
        "../day22.txt",
        "../day23.txt",
        "../day24.txt",
        "../day25.txt"
      ]
