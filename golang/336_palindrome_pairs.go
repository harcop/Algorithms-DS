package leetcode

// LeetCode #336 - Palindrome Pairs
//
// Concatenate every ordered pair and keep the indexes whose join reads the same backward.
func palindromePairs(words []string) [][]int {
	out := [][]int{}
	for i := range words {
		for j := range words {
			if i == j {
				continue
			}
			joined := words[i] + words[j]
			if bytesArePalindrome(joined) {
				out = append(out, []int{i, j})
			}
		}
	}
	return out
}

func bytesArePalindrome(word string) bool {
	left, right := 0, len(word)
	for left < right {
		right--
		if word[left] != word[right] {
			return false
		}
		left++
	}
	return true
}
