package leetcode

import "sort"

// LeetCode #260 - Single Number III
//
// LeetCode names this singleNumber; that name already belongs to the twice-repeated version (#136).
// The XOR of the two singles has a bit that splits them into two groups.
func singleNumberIII(nums []int) []int {
	xor := 0
	for _, value := range nums {
		xor ^= value
	}
	diff := xor & -xor
	a, b := 0, 0
	for _, value := range nums {
		if value&diff != 0 {
			a ^= value
		} else {
			b ^= value
		}
	}
	out := []int{a, b}
	sort.Ints(out)
	return out
}
