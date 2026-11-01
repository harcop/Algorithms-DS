package leetcode

import "sort"

// LeetCode #324 - Wiggle Sort II
//
// LeetCode names this wiggleSort; that name already belongs to the looser wiggle (#280).
// Partition around the median through a virtual index so neighbors strictly alternate.
func wiggleSortII(nums []int) {
	n := len(nums)
	if n <= 1 {
		return
	}
	sorted := append([]int(nil), nums...)
	sort.Ints(sorted)
	median := sorted[n/2]
	mapIndex := func(i int) int { return (1 + 2*i) % (n | 1) }
	i, j, k := 0, 0, n-1
	for j <= k {
		switch {
		case nums[mapIndex(j)] > median:
			a, b := mapIndex(i), mapIndex(j)
			nums[a], nums[b] = nums[b], nums[a]
			i++
			j++
		case nums[mapIndex(j)] < median:
			a, b := mapIndex(j), mapIndex(k)
			nums[a], nums[b] = nums[b], nums[a]
			k--
		default:
			j++
		}
	}
}
