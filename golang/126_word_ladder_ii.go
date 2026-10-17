package leetcode

// LeetCode #126 - Word Ladder II
//
// Breadth-first search one change at a time and keep every path that first reaches the end word.
func findLadders(beginWord string, endWord string, wordList []string) [][]string {
	dict := make(map[string]struct{}, len(wordList))
	for _, word := range wordList {
		dict[word] = struct{}{}
	}
	if _, ok := dict[endWord]; !ok {
		return nil
	}

	layer := map[string][][]string{
		beginWord: {{beginWord}},
	}
	for len(layer) > 0 {
		for word := range layer {
			delete(dict, word)
		}
		next := make(map[string][][]string)
		for word, paths := range layer {
			bytes := []byte(word)
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
					for _, path := range paths {
						copied := append(append([]string(nil), path...), neighbor)
						next[neighbor] = append(next[neighbor], copied)
					}
				}
				bytes[i] = original
			}
		}
		if paths, ok := next[endWord]; ok {
			return paths
		}
		layer = next
	}
	return nil
}
