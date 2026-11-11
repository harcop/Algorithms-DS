package leetcode

// LeetCode #491 - Non-decreasing Subsequences
//
// Build every increasing path, and skip a value that was already chosen at the same depth.
func findSubsequences(nums []int) [][]int {
	var out [][]int
	var path []int
	var walk func(start int)
	walk = func(start int) {
		if len(path) >= 2 {
			out = append(out, append([]int(nil), path...))
		}
		used := map[int]bool{}
		for i := start; i < len(nums); i++ {
			if used[nums[i]] {
				continue
			}
			if len(path) > 0 && nums[i] < path[len(path)-1] {
				continue
			}
			used[nums[i]] = true
			path = append(path, nums[i])
			walk(i + 1)
			path = path[:len(path)-1]
		}
	}
	walk(0)
	return out
}
