val c = -1.0 / math.log(2.0)
for (line <- scala.io.Source.stdin.getLines) {
  val n = line.size
  var counts = new scala.collection.mutable.HashMap[Char, Int]
  for (c <- line)
    counts.update(c, counts.getOrElse(c, 0) + 1)
  println(c * counts.values.map(_.toDouble / n).map(f => f * math.log(f)).sum)
}
