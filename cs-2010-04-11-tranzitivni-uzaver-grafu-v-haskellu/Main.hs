{-
    Project:   Transitive closure of a graph.
    Copyright: (c) 2010 by Petr Zemek <s3rvac@gmail.com>
    License:   BSD, see LICENSE for more details
-}


{-|
    Transitive closure of a graph.
-}
module Main where

import Char
import IO
import System
import Data.Array

{-|
    A graph is represented as an adjacency matrix, i.e. a rectangular matrix
    where [i, j] is True if and only if there is an edge from vertex
    i to vertex j.
-}
type Graph = Array (Int, Int) Bool

{-|
    Returns the number of vertices in a given graph.
-}
getVertexNum :: Graph -> Int
getVertexNum g = n
    where (_, (n, _)) = bounds g

{-|
    Computes the transitive closure of a given graph.

    Implemented using Warshall algorithm for the computation
    of a transitive closure of a graph
    (http://en.wikipedia.org/wiki/Floyd_Warshall#Applications_and_generalizations).
-}
transClosure :: Graph -> Graph
transClosure g = array ((1, 1), (n, n))
                       [((i, j), d!(n, i, j)) | i <- [1..n], j <- [1..n]]
    where d = array ((0, 1, 1), (n, n, n))
                    [((k, i, j), val k i j) | k <- [0..n], i <- [1..n], j <- [1..n]]
          n          = getVertexNum g
          val 0 i j = g!(i, j) -- base case (just direct connection)
          val k i j = (d!(k-1, i, j)) || ((d!(k-1, i, k)) && (d!(k-1, k, j)))

{-|
    Reads a graph from the standard input.

    Raises ioError if the graph could not be read or
    it is in invalid format.
-}
readGraph :: IO Graph
readGraph = do input <- getContents
               let linedInput = lines input
               let n = length $ getRawCells $ head linedInput
               let rawCells = getRawCells input
               let cells = map isTrueCell rawCells
               let getCell i j = cells!!(n*(i-1) + j-1)
               let inputHasValidChars = (filter isInvalidCell rawCells) == []
               let inputHasValidSize = (filter (\l -> (length (getRawCells l)) /= n) linedInput) == []
               if inputHasValidChars && inputHasValidSize
                   then return $ array ((1, 1), (n, n))
                                       [((i, j), getCell i j) | i <- [1..n], j <- [1..n]]
                   else ioError (userError "the adjacency matrix is not valid")
    where getRawCells       = filter (\c -> not $ isSpace c)
          isInvalidCell '0' = False
          isInvalidCell '1' = False
          isInvalidCell  _  = True
          isTrueCell    '0' = False
          isTrueCell     _  = True

{-|
    Prints the selected graph on the standard output.
-}
printGraph :: Graph -> IO ()
printGraph g = sequence_ $ map putStr [strAt i j | i <- [1..n], j <- [1..n]]
    where n = getVertexNum g
          cellValue True = "1"
          cellValue False = "0"
          strAt i j = (if 1 < j then " " else "") ++
                      (cellValue $ g!(i, j)) ++
                      (if j == n then "\n" else "")

{-|
    Prints the selected error message on the standard error.
-}
printError :: String -> IO ()
printError = hPutStrLn stderr

{-|
    Checks program arguments and if there is a help request,
    it prints the usage and exits. Otherwise, it just returns.
-}
checkHelp :: [String] -> IO ()
checkHelp ["-h"]     = printHelp >> exit
checkHelp ["--help"] = printHelp >> exit
checkHelp _          = return ()

{-|
    Prints program usage.
-}
printHelp :: IO ()
printHelp = putStrLn helpMsg
    where helpMsg = "Usage: closure\n" ++
                    "\n" ++
                    "Computes a transitive closure of a graph given its adjacency matrix,\n" ++
                    "which is expected on the standard input in the following format:\n" ++
                    "\n" ++
                    "a_11 a_12 ... a_1n\n" ++
                    "a_21 a_22 ... a_2n\n" ++
                    "...\n" ++
                    "a_n1 a_n2 ... a_nn\n" ++
                    "\n" ++
                    "where a_ij is 1 if there is an edge from vertex i to vertex j\n" ++
                    "and 0 otherwise.\n" ++
                    "\n" ++
                    "The result is then printed on the standard output."

{-|
    Exists with a success exit code.
-}
exit :: IO a
exit = exitWith ExitSuccess

{-|
    Exists with a failure exit code.
-}
die :: IO a
die = exitWith (ExitFailure 1)

{-|
    Runs the program.
-}
main :: IO ()
main = do args <- getArgs
          checkHelp args
          catch (do g <- readGraph
                    printGraph $ transClosure g
                    exit)
                (\err -> printError (show err) >> die)
