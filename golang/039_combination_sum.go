package leetcode

import "sort"

// LeetCode #39 - Combination Sum
//
// Sort the candidates, then reuse each number while the remaining target stays non-negative.
func combinationSum(candidates []int, target int) [][]int {
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
			if candidates[i] > remain {
				break
			}
			path = append(path, candidates[i])
			dfs(i, remain-candidates[i])
			path = path[:len(path)-1]
		}
	}

	dfs(0, target)
	return result
}
