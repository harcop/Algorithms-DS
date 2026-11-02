package leetcode

// LeetCode #357 - Count Numbers with Unique Digits
//
// Each extra digit has one fewer unused choice, starting from the ten numbers below 10.
func countNumbersWithUniqueDigits(n int) int {
	if n == 0 {
		return 1
	}
	total := 10
	current := 9
	available := 9
	for length := 2; length <= n && available > 0; length++ {
		current *= available
		total += current
		available--
	}
	return total
}
