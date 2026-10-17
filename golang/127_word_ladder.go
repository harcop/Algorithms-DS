package leetcode

// LeetCode #127 - Word Ladder
//
// Breadth-first search each one-letter change and stop at the first time the end word appears.
func ladderLength(beginWord string, endWord string, wordList []string) int {
	dict := make(map[string]struct{}, len(wordList))
	for _, word := range wordList {
		dict[word] = struct{}{}
	}
	if _, ok := dict[endWord]; !ok {
		return 0
	}

	type step struct {
		word string
		dist int
	}
	queue := []step{{beginWord, 1}}
	seen := map[string]struct{}{beginWord: {}}
	for head := 0; head < len(queue); head++ {
		current := queue[head]
		if current.word == endWord {
			return current.dist
		}
		bytes := []byte(current.word)
		for i := range bytes {
			original := bytes[i]
			for c := byte('a'); c <= 'z'; c++ {
				if c == original {
					continue
				}
				bytes[i] = c
				neighbor := string(bytes)
				if _, ok := dict[neighbor]; !ok {
					continue
				}
				if _, ok := seen[neighbor]; ok {
					continue
				}
				seen[neighbor] = struct{}{}
				queue = append(queue, step{neighbor, current.dist + 1})
			}
			bytes[i] = original
		}
	}
	return 0
}
