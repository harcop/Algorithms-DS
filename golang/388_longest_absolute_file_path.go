package leetcode

import "strings"

// LeetCode #388 - Longest Absolute File Path
//
// Each depth stores the length of the directory path above it, including the slash.
func lengthLongestPath(input string) int {
	pathLen := map[int]int{0: 0}
	best := 0
	for _, line := range strings.Split(input, "\n") {
		name := strings.TrimLeft(line, "\t")
		depth := len(line) - len(name)
		if strings.Contains(name, ".") {
			length := pathLen[depth] + len(name)
			if length > best {
				best = length
			}
			continue
		}
		pathLen[depth+1] = pathLen[depth] + len(name) + 1
	}
	return best
}
