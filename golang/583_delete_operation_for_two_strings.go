package leetcode

// LeetCode #583 - Delete Operation for Two Strings
//
// LeetCode names this minDistance; that name already belongs to edit distance (#72).
// Every character outside the longest common subsequence has to be deleted.
func minDeleteDistance(word1 string, word2 string) int {
	common := make([]int, len(word2)+1)
	for i := 1; i <= len(word1); i++ {
		diagonal := 0
		for j := 1; j <= len(word2); j++ {
			above := common[j]
			if word1[i-1] == word2[j-1] {
				common[j] = diagonal + 1
			} else {
				common[j] = max(common[j], common[j-1])
			}
			diagonal = above
		}
	}
	return len(word1) + len(word2) - 2*common[len(word2)]
}
