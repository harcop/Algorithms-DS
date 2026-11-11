package leetcode

// LeetCode #481 - Magical String
//
// The string describes its own run lengths, and the answer is how many ones appear in the prefix.
func magicalString(n int) int {
	if n <= 0 {
		return 0
	}
	if n <= 3 {
		return 1
	}
	s := []int{1, 2, 2}
	group := 2
	for len(s) < n {
		next := 3 - s[len(s)-1]
		for k := 0; k < s[group]; k++ {
			s = append(s, next)
		}
		group++
	}
	ones := 0
	for i := 0; i < n; i++ {
		if s[i] == 1 {
			ones++
		}
	}
	return ones
}
