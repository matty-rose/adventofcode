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

compareString :: String -> String -> Int
compareString a b =
  sum
    [ if (a !! i) /= (b !! i)
      then 1
      else 0
    | i <- [0 .. (length a - 1)]
    ]

calcOverlap :: String -> String -> String
calcOverlap a b = [a !! i | i <- [0 .. (length a - 1)], (a !! i) == (b !! i)]

cartesianProduct :: [a] -> [b] -> [(a, b)]
cartesianProduct xs ys =
  [(x, y) | (ix, x) <- zip [0 ..] xs, (iy, y) <- zip [0 ..] ys, iy >= ix]

p2 :: String -> String
p2 s =
  head $
  map (\x -> calcOverlap (fst x) (snd x)) $
  filter (\x -> compareString (fst x) (snd x) == 1) $
  cartesianProduct (lines s) (lines s)

main :: IO ()
main = do
  file <- readFile "input.txt"
  -- print $ p1 file
  print $ p2 file
