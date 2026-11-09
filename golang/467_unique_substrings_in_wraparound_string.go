package leetcode

// LeetCode #467 - Unique Substrings in Wraparound String
//
// A wraparound run ending at a letter covers every shorter run that ends there, so keep the longest run for each letter.
func findSubstringInWraproundString(p string) int {
	longest := [26]int{}
	current := 0
	for i := 0; i < len(p); i++ {
		if i > 0 && (p[i-1]+1 == p[i] || (p[i-1] == 'z' && p[i] == 'a')) {
			current++
		} else {
			current = 1
		}
		letter := p[i] - 'a'
		if current > longest[letter] {
			longest[letter] = current
		}
	}
	total := 0
	for _, length := range longest {
		total += length
	}
	return total
}
