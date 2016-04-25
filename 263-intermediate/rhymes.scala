import scala.io.{Codec,Source}

def findVowels: Iterator[String] = {
  for {
    line <- Source.fromFile("cmudict-0.7b.phones").getLines
    splitLine = line.split('\t')
    if splitLine.size == 2
    sound = splitLine(0)
    soundType = splitLine(1)
    if soundType == "vowel"
  } yield sound
}

def pronounciations(): Iterator[(String, String)] = {
  for {
    line <- Source.fromFile("cmudict-0.7b", "ISO-8859-1").getLines
    if line(0).isLetter
    splitLine = line.split("  ")
    if splitLine.size == 2
    word = splitLine(0)
    pronounciation = splitLine(1)
  } yield (word, pronounciation)
}

def findPronounciation(input: String): String = {
  for {
    (word, sounds) <- pronounciations()
    if word == input
  } return sounds
  sys.error(s"Could not find word: $input")
}

def dropStress(sound: String): String =
  if (sound.last.isDigit) sound.init else sound

def rhyming(input: String,
            vowels: Array[String],
            ignoreStress: Boolean): Iterator[(Int, String, String)] = {
  val prepareList =
    if (ignoreStress) (_: String).split(' ').map(dropStress).reverse.toList
    else (_: String).split(' ').reverse.toList
  val inputList = prepareList(input)
  val mininumMatching =
    inputList.indexWhere(sound => vowels.contains(dropStress(sound))) + 1
  for {
    (word, sounds) <- pronounciations()
    soundsList = prepareList(sounds)
    matching = (inputList zip soundsList).takeWhile{ case (a,b) => a == b }.size
    if matching >= mininumMatching
    if sounds != input
  } yield (matching, word, sounds)
}

def printRhyming(word: String, ignoreStress: Boolean) {
  val result = rhyming(findPronounciation(word.toUpperCase),
                       findVowels.toArray,
                       ignoreStress).toArray.sortBy{ case (a,b,c) => (-a, b) }
  for ((count, word, pronounciation) <- result) {
    println(s"[$count] $word $pronounciation")
  }
}

args.toList match {
  case "--ignore-stress" :: word :: Nil => printRhyming(word, true)
  case word :: Nil => printRhyming(word, false)
  case _ => println("Usage: scala rhymes.scala [--ignore-stress] word")
}
