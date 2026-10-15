package leetcode

import "strings"

// LeetCode #65 - Valid Number
//
// A number is digits with an optional sign and dot, optionally followed by an exponent.
func isNumber(s string) bool {
	s = strings.TrimSpace(s)
	if s == "" {
		return false
	}

	seenDigit := false
	seenDot := false
	seenExp := false
	digitAfterExp := true

	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
			seenDigit = true
			if seenExp {
				digitAfterExp = true
			}
		case '+', '-':
			if i != 0 && s[i-1] != 'e' && s[i-1] != 'E' {
				return false
			}
		case '.':
			if seenDot || seenExp {
				return false
			}
			seenDot = true
		case 'e', 'E':
			if seenExp || !seenDigit {
				return false
			}
			seenExp = true
			digitAfterExp = false
		default:
			return false
		}
	}

	return seenDigit && digitAfterExp
}
