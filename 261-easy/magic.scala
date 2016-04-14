def is_magic_square(numbers: Array[Int]) : Boolean = {
  val ranges = List((0 to 2), (3 to 5), (6 to 8),
                    (0 to 6 by 3), (1 to 7 by 3), (2 to 8 by 3),
                    (0 to 8 by 4), (2 to 6 by 2))
  ranges.forall(_.map(numbers).sum == 15)
}

assert(is_magic_square(Array(8, 1, 6, 3, 5, 7, 4, 9, 2)))
assert(is_magic_square(Array(2, 7, 6, 9, 5, 1, 4, 3, 8)))
assert( ! is_magic_square(Array(3, 5, 7, 8, 1, 6, 4, 9, 2)))
assert( ! is_magic_square(Array(8, 1, 6, 7, 5, 3, 4, 9, 2)))

def can_be_magic_square(top_two_rows: Array[Int]): Boolean = {
  (1 to 9).filterNot(top_two_rows.contains).permutations.exists {
    last_row => is_magic_square(top_two_rows ++ last_row)
  }
}

assert(can_be_magic_square(Array(8, 1, 6, 3, 5, 7)))
assert(can_be_magic_square(Array(2, 7, 6, 9, 5, 1)))
assert( ! can_be_magic_square(Array(3, 5, 7, 8, 1, 6)))
assert( ! can_be_magic_square(Array(8, 1, 6, 7, 5, 3)))
