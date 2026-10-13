package leetcode

// LeetCode #17 - Letter Combinations of a Phone Number
//
// Build every combination by appending the letters mapped to each digit.
func letterCombinations(digits string) []string {
	if digits == "" {
		return []string{}
	}

	phone := []string{"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"}
	result := []string{""}

	for i := 0; i < len(digits); i++ {
		letters := phone[digits[i]-'0']
		next := make([]string, 0, len(result)*len(letters))
		for _, prefix := range result {
			for j := 0; j < len(letters); j++ {
				next = append(next, prefix+string(letters[j]))
			}
		}
		result = next
	}

	return result
}
