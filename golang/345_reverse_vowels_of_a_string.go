package leetcode

// LeetCode #345 - Reverse Vowels of a String
//
// Swap the next vowel from each end and leave every other character in place.
func reverseVowels(s string) string {
	chars := []byte(s)
	isVowel := func(c byte) bool {
		switch c {
		case 'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U':
			return true
		default:
			return false
		}
	}
	i, j := 0, len(chars)-1
	for i < j {
		for i < j && !isVowel(chars[i]) {
			i++
		}
		for i < j && !isVowel(chars[j]) {
			j--
		}
		if i < j {
			chars[i], chars[j] = chars[j], chars[i]
			i++
			j--
		}
	}
	return string(chars)
}
