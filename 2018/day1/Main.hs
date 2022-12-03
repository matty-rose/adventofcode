import qualified Data.Set as Set

parseNum :: String -> Integer
parseNum ('+':num) = read num :: Integer
parseNum ('-':num) = -1 * read num :: Integer
parseNum _ = 0

p1 :: String -> Integer
p1 s = foldl (+) 0 $ map parseNum $ lines s

detectDup :: [Integer] -> Maybe Integer
detectDup = go Set.empty
  where
    go _ [] = Nothing
    go z (x:xs)
      | x `Set.member` z = Just x
      | otherwise = go (Set.insert x z) xs

p2 :: String -> Maybe Integer
p2 s = detectDup $ scanl (+) 0 $ map parseNum $ cycle $ lines s

main :: IO ()
main = do
  file <- readFile "input.txt"
  print $ p1 file
  print $ p2 file
