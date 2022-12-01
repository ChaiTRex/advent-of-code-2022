module AoC where

import Data.Word         (Word8)
import System.IO         (readFile)
import System.IO.Unsafe  (unsafePerformIO)

{-# INLINE day #-}
day :: Word8 -> String
day d = let pathPrefix = if      d >=  1 && d <=  9 then "../day0"
                         else if d >= 10 && d <= 25 then "../day"
                         else                            error "`d` is not between 1 and 25"
            path       = showString pathPrefix . shows d $ ".txt"
        in  unsafePerformIO . readFile $ path
