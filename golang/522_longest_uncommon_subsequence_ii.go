package leetcode

import "sort"

// LeetCode #522 - Longest Uncommon Subsequence II
//
// LeetCode names this findLUSlength; that name already belongs to the two-string version (#521).
// The longest word that is not a subsequence of any other word is the answer.
func findLUSlengthII(strs []string) int {
	sort.Slice(strs, func(i, j int) bool { return len(strs[i]) > len(strs[j]) })
	for i := range strs {
		uncommon := true
		for j := range strs {
			if i != j && isSubsequence(strs[i], strs[j]) {
				uncommon = false
				break
			}
		}
		if uncommon {
			return len(strs[i])
		}
	}
	return -1
}
