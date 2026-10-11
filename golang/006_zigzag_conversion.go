package leetcode

import "strings"

// LeetCode #6 - Zigzag Conversion
//
// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows;
// read line by line and return the result string.
func convert(s string, numRows int) string {
	if numRows <= 1 || len(s) <= numRows {
		return s
	}

	rows := make([]string, numRows)
	cur := 0
	step := 1

	for _, c := range s {
		rows[cur] += string(c)
		if cur == 0 {
			step = 1
		} else if cur == numRows-1 {
			step = -1
		}
		cur += step
	}

	return strings.Join(rows, "")
}
