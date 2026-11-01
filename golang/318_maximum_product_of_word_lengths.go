package leetcode

// LeetCode #318 - Maximum Product of Word Lengths
//
// LeetCode names this maxProduct; that name already belongs to the subarray product (#152).
// Bitmasks show which letters a word uses, so disjoint words can be multiplied.
func maxProductWords(words []string) int {
	masks := make([]uint32, len(words))
	lengths := make([]int, len(words))
	for i, word := range words {
		var mask uint32
		for j := 0; j < len(word); j++ {
			mask |= 1 << (word[j] - 'a')
		}
		masks[i] = mask
		lengths[i] = len(word)
	}
	best := 0
	for i := 0; i < len(words); i++ {
		for j := i + 1; j < len(words); j++ {
			if masks[i]&masks[j] == 0 && lengths[i]*lengths[j] > best {
				best = lengths[i] * lengths[j]
			}
		}
	}
	return best
}
