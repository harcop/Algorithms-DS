package leetcode

// LeetCode #676 - Implement Magic Dictionary
//
// A search hits when exactly one character differs from a stored word of the same length.
type MagicDictionary struct {
	words []string
}

func NewMagicDictionary() *MagicDictionary {
	return &MagicDictionary{}
}

func (m *MagicDictionary) BuildDict(dictionary []string) {
	m.words = append([]string(nil), dictionary...)
}

func (m *MagicDictionary) Search(searchWord string) bool {
	for _, word := range m.words {
		if len(word) != len(searchWord) {
			continue
		}
		diff := 0
		for i := 0; i < len(word); i++ {
			if word[i] != searchWord[i] {
				diff++
			}
		}
		if diff == 1 {
			return true
		}
	}
	return false
}
