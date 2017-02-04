package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var notes = [12]string{"C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"}

// A Note represents a note as an integer where e.g. C2 => 2*12 + 0, C#3 => 3*12 + 1.
type Note int

// Increment adds i semitones to the note's pitch.
func (n Note) Increment(i int) Note {
	return Note(int(n) + i)
}

// String formats a note in the usual format, e.g. "C#3".
func (n Note) String() string {
	return fmt.Sprintf("%s%d", notes[n%12], n/12)
}

// guitarStrings contains the notes played by the 6 guitar strings when no fret is pressed.
var guitarStrings = [6]Note{52, 47, 43, 38, 33, 28}

func isDigit(b byte) bool {
	return b >= '0' && b <= '9'
}

func main() {
	var output []string

	// read tablature from stdin
	var lines []string
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	// look at input column-by-column
	nColumns := len(lines[0])
	for i := 0; i < nColumns; i++ {

		// see if any line has a digit in column i
		for j, line := range lines {
			if isDigit(line[i]) {
				digits := 1
				if isDigit(line[i+1]) {
					digits++
				}
				n, _ := strconv.Atoi(line[i : i+digits])
				output = append(output, guitarStrings[j].Increment(n).String())
				i += digits
			}
		}
	}

	// print output
	fmt.Println(strings.Join(output, " "))
}
