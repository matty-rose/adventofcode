import Data.List
import qualified Data.Map

getCharCounts :: String -> [Int]
getCharCounts s =
  map snd $ Data.Map.toList $ Data.Map.fromListWith (+) [(c, 1) | c <- s]

containsCount :: Int -> [Int] -> Int
containsCount n counts = fromEnum $ elem n counts

mulTuple :: (Int, Int) -> Int
mulTuple (x, y) = (*) x y

p1 :: String -> Int
p1 s =
  mulTuple $
  foldl (\a b -> (fst a + fst b, snd a + snd b)) (0, 0) $
  map (\x -> (containsCount 2 x, containsCount 3 x)) $
  map getCharCounts $ lines s

p2 :: String -> [String]
p2 s = lines s

main :: IO ()
main = do
  file <- readFile "input.txt"
  print $ p1 file
  -- print $ p2 file
