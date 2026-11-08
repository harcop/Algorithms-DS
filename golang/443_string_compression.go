package leetcode

import "strconv"

// LeetCode #443 - String Compression
//
// Write each run as its character and, when the run is longer than one, the digits of its length.
func compress(chars []byte) int {
	write := 0
	for i := 0; i < len(chars); {
		j := i + 1
		for j < len(chars) && chars[j] == chars[i] {
			j++
		}
		chars[write] = chars[i]
		write++
		if length := j - i; length > 1 {
			digits := strconv.Itoa(length)
			for k := 0; k < len(digits); k++ {
				chars[write] = digits[k]
				write++
			}
		}
		i = j
	}
	return write
}
