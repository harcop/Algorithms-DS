package leetcode

import "sort"

// LeetCode #40 - Combination Sum II
//
// Each candidate is used at most once. After sorting, skip duplicate values at the same depth.
func combinationSum2(candidates []int, target int) [][]int {
	sort.Ints(candidates)
	var result [][]int
	var path []int

	var dfs func(start, remain int)
	dfs = func(start, remain int) {
		if remain == 0 {
			result = append(result, append([]int(nil), path...))
			return
		}
		for i := start; i < len(candidates); i++ {
			if i > start && candidates[i] == candidates[i-1] {
				continue
			}
			if candidates[i] > remain {
				break
			}
			path = append(path, candidates[i])
			dfs(i+1, remain-candidates[i])
			path = path[:len(path)-1]
		}
	}

	dfs(0, target)
	return result
}
