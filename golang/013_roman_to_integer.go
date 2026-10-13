package leetcode

// LeetCode #13 - Roman to Integer
//
// A smaller value immediately before a larger one is subtracted; otherwise it is added.
func romanToInt(s string) int {
	values := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}

	total := 0
	for i := 0; i < len(s); i++ {
		current := values[s[i]]
		if i+1 < len(s) && current < values[s[i+1]] {
			total -= current
		} else {
			total += current
		}
	}
	return total
}
