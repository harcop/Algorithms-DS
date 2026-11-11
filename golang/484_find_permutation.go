package leetcode

// LeetCode #484 - Find Permutation
//
// Start from 1, 2, 3, and reverse each run of D so those positions decrease.
func findPermutation(s string) []int {
	perm := make([]int, len(s)+1)
	for i := range perm {
		perm[i] = i + 1
	}
	for i := 0; i < len(s); {
		if s[i] != 'D' {
			i++
			continue
		}
		start := i
		for i < len(s) && s[i] == 'D' {
			i++
		}
		for left, right := start, i; left < right; left, right = left+1, right-1 {
			perm[left], perm[right] = perm[right], perm[left]
		}
	}
	return perm
}
