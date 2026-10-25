package leetcode

// LeetCode #269 - Alien Dictionary
//
// The first differing letters of neighboring words are edges, and a cycle means no order exists.
func alienOrder(words []string) string {
	next := map[byte]map[byte]struct{}{}
	indegree := map[byte]int{}
	for _, word := range words {
		for i := 0; i < len(word); i++ {
			if _, ok := indegree[word[i]]; !ok {
				indegree[word[i]] = 0
				next[word[i]] = map[byte]struct{}{}
			}
		}
	}
	for i := 0; i+1 < len(words); i++ {
		a, b := words[i], words[i+1]
		found := false
		limit := len(a)
		if len(b) < limit {
			limit = len(b)
		}
		for j := 0; j < limit; j++ {
			if a[j] == b[j] {
				continue
			}
			if _, ok := next[a[j]][b[j]]; !ok {
				next[a[j]][b[j]] = struct{}{}
				indegree[b[j]]++
			}
			found = true
			break
		}
		if !found && len(a) > len(b) {
			return ""
		}
	}
	var queue []byte
	for letter, degree := range indegree {
		if degree == 0 {
			queue = append(queue, letter)
		}
	}
	order := make([]byte, 0, len(indegree))
	for head := 0; head < len(queue); head++ {
		letter := queue[head]
		order = append(order, letter)
		for nextLetter := range next[letter] {
			indegree[nextLetter]--
			if indegree[nextLetter] == 0 {
				queue = append(queue, nextLetter)
			}
		}
	}
	if len(order) != len(indegree) {
		return ""
	}
	return string(order)
}
