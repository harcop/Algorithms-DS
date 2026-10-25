package leetcode

// LeetCode #244 - Shortest Word Distance II
//
// Store every index up front, then walk the two sorted index lists once per query.
type WordDistance struct {
	index map[string][]int
}

func NewWordDistance(wordsDict []string) WordDistance {
	index := map[string][]int{}
	for i, word := range wordsDict {
		index[word] = append(index[word], i)
	}
	return WordDistance{index: index}
}

func (words WordDistance) Shortest(word1 string, word2 string) int {
	if word1 == word2 {
		positions := words.index[word1]
		if len(positions) < 2 {
			return 0
		}
		best := positions[1] - positions[0]
		for i := 2; i < len(positions); i++ {
			if gap := positions[i] - positions[i-1]; gap < best {
				best = gap
			}
		}
		return best
	}
	a := words.index[word1]
	b := words.index[word2]
	i, j, best := 0, 0, len(a)+len(b)
	for i < len(a) && j < len(b) {
		gap := a[i] - b[j]
		if gap < 0 {
			gap = -gap
		}
		if gap < best {
			best = gap
		}
		if a[i] < b[j] {
			i++
		} else {
			j++
		}
	}
	return best
}
