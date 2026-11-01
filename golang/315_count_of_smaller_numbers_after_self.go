package leetcode

import "sort"

// LeetCode #315 - Count of Smaller Numbers After Self
//
// Walk from the right and use a Fenwick tree to count how many smaller values are already stored.
func countSmaller(nums []int) []int {
	if len(nums) == 0 {
		return nil
	}
	sorted := append([]int(nil), nums...)
	sort.Ints(sorted)
	unique := []int{sorted[0]}
	for _, value := range sorted[1:] {
		if value != unique[len(unique)-1] {
			unique = append(unique, value)
		}
	}
	bit := make([]int, len(unique)+2)
	add := func(index, value int) {
		for index < len(bit) {
			bit[index] += value
			index += index & -index
		}
	}
	prefix := func(index int) int {
		sum := 0
		for index > 0 {
			sum += bit[index]
			index -= index & -index
		}
		return sum
	}
	out := make([]int, len(nums))
	for i := len(nums) - 1; i >= 0; i-- {
		rank := sort.SearchInts(unique, nums[i]) + 1
		out[i] = prefix(rank - 1)
		add(rank, 1)
	}
	return out
}
