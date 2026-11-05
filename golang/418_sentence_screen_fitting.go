package leetcode

// LeetCode #418 - Sentence Screen Fitting
//
// Join the sentence into a repeating line, then back up to a space whenever a row cuts through a word.
func wordsTyping(sentence []string, rows int, cols int) int {
	if len(sentence) == 0 || rows == 0 || cols == 0 {
		return 0
	}
	line := sentence[0]
	for _, word := range sentence[1:] {
		line += " " + word
	}
	line += " "
	n := len(line)
	pos := 0
	for row := 0; row < rows; row++ {
		pos += cols
		if line[pos%n] == ' ' {
			pos++
			continue
		}
		for pos > 0 && line[(pos-1)%n] != ' ' {
			pos--
		}
	}
	return pos / n
}
