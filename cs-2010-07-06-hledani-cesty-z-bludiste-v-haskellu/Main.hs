{-
    Project:   A very simple maze solver
    Copyright: (c) 2010 by Petr Zemek <s3rvac@gmail.com>
    License:   BSD, see LICENSE for more details
-}

{-|
    A very simple maze solver.
-}
module Main where

import System
import Array

{-|
    A maze is represented as a two dimensional matrix
    with the following cell types:
        '#' - wall
        '.' - free path
        'S' - start cell
        'G' - goal cell
        '+' - path from start to goal in a solved maze
-}
type Maze = Array (Int, Int) Char

{-|
    Finds a way through the selected maze.

    Returns Just Maze if some way was found, Nothing otherwise.
-}
findWayThrough :: Maze -> Maybe Maze
findWayThrough maze = solveMaze maze i j
    where (i, j) = findStart maze

{-|
    Finds a place (i, j) from which the maze solving
    has to start.

    This is the place up, down, right, or left
    to the start cell.
-}
findStart :: Maze -> (Int, Int)
findStart maze = find cells
    where (_, (m, _)) = bounds maze
          cells = assocs maze
          find []                = error "no start point"
          find (((i, j), 'S'):_) | i == 1    = (i+1, j)
                                 | i == m    = (i-1, j)
                                 | j == 1    = (i, j+1)
                                 | otherwise = (i, j-1)
          find (_:xs) = find xs

{-|
    Solves the selected maze using depth-first search
    starting from the selected location i j.

    Returns Just Maze if some way was found, Nothing otherwise.
-}
solveMaze :: Maze -> Int -> Int -> Maybe Maze
solveMaze maze i j | currCell == 'G' = Just maze
                   | currCell == '.' = nextMove
                   | otherwise       = Nothing
    where currCell = maze!(i, j)
          nextMove | isOk goU  = goU
                   | isOk goR  = goR
                   | isOk goD  = goD
                   | otherwise = goL
          goU = solveMaze newMaze (i-1) j
          goD = solveMaze newMaze (i+1) j
          goR = solveMaze newMaze i (j+1)
          goL = solveMaze newMaze i (j-1)
          newMaze = maze // [((i, j), '+')]
          isOk (Just _) = True
          isOk Nothing  = False

{-|
    Reads a maze from the standard input.
-}
readMaze :: IO Maze
readMaze = do input <- getContents
              let linedInput = lines input
              let m = length linedInput
              let n = length $ head linedInput
              let cells = filter ('\n' /=) input
              let getCell i j = cells!!(n*(i-1) + j-1)
              return $ array ((1, 1), (m, n))
                             [((i, j), getCell i j) | i <- [1..m], j <- [1..n]]

{-|
    Prints the result of a search trough a maze
    on the standard output.
-}
printResult :: Maybe Maze -> IO ()
printResult Nothing     = putStrLn "no solution was found"
printResult (Just maze) = sequence_ $ map putStr [strAt i j | i <- [1..m], j <- [1..n]]
    where (_, (m, n)) = bounds maze
          strAt i j = (maze!(i, j)) : nl
              where nl | j == n    = "\n"
                       | otherwise = ""

{-|
    Checks program arguments and if there is a help request,
    it prints program usage and exits. Otherwise, it just returns.
-}
checkHelp :: [String] -> IO ()
checkHelp ["-h"]     = printHelp
checkHelp ["--help"] = printHelp
checkHelp _          = return ()

{-|
    Prints program usage and exits.
-}
printHelp :: IO ()
printHelp = do putStrLn helpMsg
               exitWith ExitSuccess
    where helpMsg = "Usage: maze\n" ++
                "\n" ++
                "Finds a way from S to G in a maze given on the standard\n" ++
                "input in the following text format:\n" ++
                "\n" ++
                "    ######G####\n" ++
                "    #...#..#.##\n" ++
                "    ##.##.#...#\n" ++
                "    #.....#.#.#\n" ++
                "    #S#########\n"

{-|
    Runs the program.
-}
main :: IO ()
main = do args <- getArgs
          checkHelp args
          maze <- readMaze
          printResult $ findWayThrough maze
          exitWith ExitSuccess
