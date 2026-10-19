package leetcode

import "strconv"

// LeetCode #165 - Compare Version Numbers
//
// Compare the numbers between dots, treating a missing part as zero.
func compareVersion(version1 string, version2 string) int {
	parts1 := splitVersion(version1)
	parts2 := splitVersion(version2)
	n := len(parts1)
	if len(parts2) > n {
		n = len(parts2)
	}
	for i := 0; i < n; i++ {
		a, b := 0, 0
		if i < len(parts1) {
			a = parts1[i]
		}
		if i < len(parts2) {
			b = parts2[i]
		}
		if a < b {
			return -1
		}
		if a > b {
			return 1
		}
	}
	return 0
}

func splitVersion(version string) []int {
	var parts []int
	start := 0
	for i := 0; i <= len(version); i++ {
		if i == len(version) || version[i] == '.' {
			value, _ := strconv.Atoi(version[start:i])
			parts = append(parts, value)
			start = i + 1
		}
	}
	return parts
}
