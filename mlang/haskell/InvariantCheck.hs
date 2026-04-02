module Main where

import Data.List (tails)
import System.Environment (getArgs)
import Text.Read (readMaybe)

data Snapshot = Snapshot
  { snapTick :: Integer
  , snapPosition :: Integer
  , snapVelocity :: Integer
  , snapEnergy :: Integer
  , snapAccumulator :: Integer
  }
  deriving (Eq, Show)

parseSnapshot :: String -> Maybe Snapshot
parseSnapshot line =
  case splitComma line of
    [tickValue, positionValue, velocityValue, energyValue, accumulatorValue] ->
      Snapshot
        <$> readMaybe tickValue
        <*> readMaybe positionValue
        <*> readMaybe velocityValue
        <*> readMaybe energyValue
        <*> readMaybe accumulatorValue
    _ -> Nothing

splitComma :: String -> [String]
splitComma [] = [""]
splitComma (',' : rest) = "" : splitComma rest
splitComma (character : rest) =
  let values = splitComma rest
   in (character : head values) : tail values

tickMonotonic :: [Snapshot] -> Bool
tickMonotonic snapshots =
  all (\pair -> snapTick (head pair) < snapTick (pair !! 1)) (filter ((>= 2) . length) (map (take 2) (tails snapshots)))

energyNonNegative :: [Snapshot] -> Bool
energyNonNegative = all (\snapshot -> snapEnergy snapshot >= 0)

velocityBounded :: Integer -> [Snapshot] -> Bool
velocityBounded limit = all (\snapshot -> abs (snapVelocity snapshot) <= limit)

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      contents <- readFile path
      let snapshots = mapM parseSnapshot (filter (not . null) (lines contents))
      case snapshots of
        Nothing -> putStrLn "invalid snapshot stream"
        Just values -> do
          putStrLn ("snapshots=" ++ show (length values))
          putStrLn ("tick_monotonic=" ++ show (tickMonotonic values))
          putStrLn ("energy_non_negative=" ++ show (energyNonNegative values))
          putStrLn ("velocity_bounded_4096=" ++ show (velocityBounded 4096 values))
    _ -> putStrLn "usage: InvariantCheck <snapshot-stream.csv>"
