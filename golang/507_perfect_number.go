package leetcode

// LeetCode #507 - Perfect Number
//
// A number is perfect when the proper divisors, found in pairs up to its square root, add up to it.
func checkPerfectNumber(num int) bool {
	if num <= 1 {
		return false
	}
	sum := 1
	for divisor := 2; divisor*divisor <= num; divisor++ {
		if num%divisor != 0 {
			continue
		}
		sum += divisor
		if other := num / divisor; other != divisor {
			sum += other
		}
	}
	return sum == num
}
