package leetcode

// LeetCode #386 - Lexicographical Numbers
//
// Depth-first search appends a digit while the number stays within n.
func lexicalOrder(n int) []int {
	out := make([]int, 0, n)
	var walk func(int)
	walk = func(cur int) {
		if cur > n {
			return
		}
		out = append(out, cur)
		for digit := 0; digit <= 9; digit++ {
			next := cur*10 + digit
			if next > n {
				return
			}
			walk(next)
		}
	}
	for digit := 1; digit <= 9 && digit <= n; digit++ {
		walk(digit)
	}
	return out
}
