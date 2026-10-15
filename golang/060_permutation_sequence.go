package leetcode

// LeetCode #60 - Permutation Sequence
//
// Factorials split the remaining permutations into equal blocks, so each digit is chosen directly.
func getPermutation(n int, k int) string {
	nums := make([]int, n)
	for i := range nums {
		nums[i] = i + 1
	}
	fact := make([]int, n+1)
	fact[0] = 1
	for i := 1; i <= n; i++ {
		fact[i] = fact[i-1] * i
	}

	k--
	out := make([]byte, 0, n)
	for length := n; length >= 1; length-- {
		idx := k / fact[length-1]
		k %= fact[length-1]
		out = append(out, byte('0'+nums[idx]))
		nums = append(nums[:idx], nums[idx+1:]...)
	}
	return string(out)
}
