package leetcode

// LeetCode #414 - Third Maximum Number
//
// Keep the three distinct maxima, and return the largest when a third one never appears.
func thirdMax(nums []int) int {
	const missing = int64(1 << 62)
	first, second, third := missing, missing, missing
	for _, num := range nums {
		value := int64(num)
		if value == first || value == second || value == third {
			continue
		}
		switch {
		case first == missing || value > first:
			first, second, third = value, first, second
		case second == missing || value > second:
			second, third = value, second
		case third == missing || value > third:
			third = value
		}
	}
	if third == missing {
		return int(first)
	}
	return int(third)
}
