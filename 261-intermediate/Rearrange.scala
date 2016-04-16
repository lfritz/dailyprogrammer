import scala.io.Source

def read(filename: String) =
  Source.fromFile(filename).getLines.map(_.split(" ").map(_.toInt)).toArray

def print(numbers: Array[Array[Int]]) =
  println(numbers.map(_.mkString(" ")).mkString("\n"))

def rearrangeMagically(input: Array[Array[Int]]): Array[Array[Int]] = {
  val n = input.length
  val expectedSum = n * (n*n + 1) / 2
  for (order <- (0 until n).toArray.permutations) {
    if ((0 until n).map(i => input(order(i))(i)).sum == expectedSum &&
        (0 until n).map(i => input(order(i))(n-1-i)).sum == expectedSum)
      return order.map(input).toArray
  }
  throw new Exception("could not create magic square!")
}

print(rearrangeMagically(read(args(0))))
