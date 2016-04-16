def isMagicSquare(numbers: Array[Int]) : Boolean = {
  val ranges = List((0 to 2), (3 to 5), (6 to 8),
                    (0 to 6 by 3), (1 to 7 by 3), (2 to 8 by 3),
                    (0 to 8 by 4), (2 to 6 by 2))
  ranges.forall(_.map(numbers).sum == 15)
}

assert(isMagicSquare(Array(8, 1, 6, 3, 5, 7, 4, 9, 2)))
assert(isMagicSquare(Array(2, 7, 6, 9, 5, 1, 4, 3, 8)))
assert( ! isMagicSquare(Array(3, 5, 7, 8, 1, 6, 4, 9, 2)))
assert( ! isMagicSquare(Array(8, 1, 6, 7, 5, 3, 4, 9, 2)))

def canBeMagicSquare(topTwoRows: Array[Int]): Boolean = {
  (1 to 9).filterNot(topTwoRows.contains).permutations.exists {
    lastRow => isMagicSquare(topTwoRows ++ lastRow)
  }
}

assert(canBeMagicSquare(Array(8, 1, 6, 3, 5, 7)))
assert(canBeMagicSquare(Array(2, 7, 6, 9, 5, 1)))
assert( ! canBeMagicSquare(Array(3, 5, 7, 8, 1, 6)))
assert( ! canBeMagicSquare(Array(8, 1, 6, 7, 5, 3)))
