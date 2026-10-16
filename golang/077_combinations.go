package leetcode

// LeetCode #77 - Combinations
//
// Choose the next number from the remaining range and stop when the path has length k.
func combine(n int, k int) [][]int {
	var result [][]int
	var path []int

	var dfs func(start int)
	dfs = func(start int) {
		if len(path) == k {
			result = append(result, append([]int(nil), path...))
			return
		}
		for i := start; i <= n; i++ {
			path = append(path, i)
			dfs(i + 1)
			path = path[:len(path)-1]
		}
	}

	dfs(1)
	return result
}
