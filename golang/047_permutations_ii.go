package leetcode

import "sort"

// LeetCode #47 - Permutations II
//
// Sort first, then skip a duplicate value unless its earlier copy is already in the path.
func permuteUnique(nums []int) [][]int {
	sort.Ints(nums)
	used := make([]bool, len(nums))
	var path []int
	var result [][]int

	var dfs func()
	dfs = func() {
		if len(path) == len(nums) {
			result = append(result, append([]int(nil), path...))
			return
		}
		for i := 0; i < len(nums); i++ {
			if used[i] {
				continue
			}
			if i > 0 && nums[i] == nums[i-1] && !used[i-1] {
				continue
			}
			used[i] = true
			path = append(path, nums[i])
			dfs()
			path = path[:len(path)-1]
			used[i] = false
		}
	}

	dfs()
	return result
}
