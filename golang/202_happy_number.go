package leetcode

// LeetCode #202 - Happy Number
//
// Replace the number with the sum of the squares of its digits until 1 or a repeat.
func isHappy(n int) bool {
	seen := map[int]struct{}{}
	for n != 1 {
		if _, ok := seen[n]; ok {
			return false
		}
		seen[n] = struct{}{}
		sum := 0
		for n > 0 {
			digit := n % 10
			sum += digit * digit
			n /= 10
		}
		n = sum
	}
	return true
}
