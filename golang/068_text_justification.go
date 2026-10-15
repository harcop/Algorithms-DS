package leetcode

import "strings"

// LeetCode #68 - Text Justification
//
// Fill each line, then spread leftover spaces across the gaps. The last line stays left aligned.
func fullJustify(words []string, maxWidth int) []string {
	var result []string
	var line []string
	lineLen := 0

	for _, word := range words {
		extra := 0
		if len(line) > 0 {
			extra = 1
		}
		if lineLen+extra+len(word) > maxWidth {
			result = append(result, packLine(line, maxWidth, false))
			line = nil
			lineLen = 0
		}
		if len(line) > 0 {
			lineLen++
		}
		lineLen += len(word)
		line = append(line, word)
	}
	if len(line) > 0 {
		result = append(result, packLine(line, maxWidth, true))
	}
	return result
}

func packLine(words []string, maxWidth int, lastLine bool) string {
	if len(words) == 1 || lastLine {
		line := strings.Join(words, " ")
		if len(line) < maxWidth {
			line += strings.Repeat(" ", maxWidth-len(line))
		}
		return line
	}

	chars := 0
	for _, word := range words {
		chars += len(word)
	}
	gaps := len(words) - 1
	spaces := maxWidth - chars
	base := spaces / gaps
	extra := spaces % gaps

	var b strings.Builder
	for i, word := range words {
		b.WriteString(word)
		if i < gaps {
			count := base
			if i < extra {
				count++
			}
			b.WriteString(strings.Repeat(" ", count))
		}
	}
	return b.String()
}
