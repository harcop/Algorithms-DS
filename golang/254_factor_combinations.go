package leetcode

// LeetCode #254 - Factor Combinations
//
// Multiply integers from 2 upward, keeping factors in nondecreasing order.
func getFactors(n int) [][]int {
	var out [][]int
	var path []int
	var choose func(start, target int)
	choose = func(start, target int) {
		if target > 1 && len(path) > 0 {
			combo := append([]int(nil), path...)
			out = append(out, append(combo, target))
		}
		for factor := start; factor <= target/factor; factor++ {
			if target%factor != 0 {
				continue
			}
			path = append(path, factor)
			choose(factor, target/factor)
			path = path[:len(path)-1]
		}
	}
	choose(2, n)
	return out
}
