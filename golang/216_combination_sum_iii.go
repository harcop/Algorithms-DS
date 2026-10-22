package leetcode

// LeetCode #216 - Combination Sum III
//
// Choose k distinct digits from 1 to 9 whose sum is n.
func combinationSum3(k int, n int) [][]int {
	var out [][]int
	var path []int
	var choose func(start, left, sum int)
	choose = func(start, left, sum int) {
		if left == 0 && sum == n {
			out = append(out, append([]int(nil), path...))
			return
		}
		if left == 0 || sum > n {
			return
		}
		for value := start; value <= 9; value++ {
			if sum+value > n {
				break
			}
			path = append(path, value)
			choose(value+1, left-1, sum+value)
			path = path[:len(path)-1]
		}
	}
	choose(1, k, 0)
	return out
}
