package leetcode

import "sort"

// LeetCode #455 - Assign Cookies
//
// Give the smallest cookie that still satisfies the smallest remaining child.
func findContentChildren(g []int, s []int) int {
	sort.Ints(g)
	sort.Ints(s)
	child := 0
	for cookie := 0; child < len(g) && cookie < len(s); cookie++ {
		if s[cookie] >= g[child] {
			child++
		}
	}
	return child
}
