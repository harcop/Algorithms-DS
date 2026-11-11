package leetcode

// LeetCode #500 - Keyboard Row
//
// LeetCode names this findWords; that name already belongs to the board search (#212).
// A word stays when every letter comes from the same keyboard row.
func findKeyboardWords(words []string) []string {
	var out []string
	for _, word := range words {
		if word == "" || oneKeyboardRow(word) {
			out = append(out, word)
		}
	}
	return out
}

func oneKeyboardRow(word string) bool {
	row := keyboardRow(word[0])
	for i := 1; i < len(word); i++ {
		if keyboardRow(word[i]) != row {
			return false
		}
	}
	return true
}

func keyboardRow(c byte) int {
	if c >= 'A' && c <= 'Z' {
		c += 'a' - 'A'
	}
	switch c {
	case 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p':
		return 1
	case 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l':
		return 2
	default:
		return 3
	}
}
