package leetcode

import "math"

// LeetCode #625 - Minimum Factorization
//
// Factor into digits from 9 down to 2, then place the smaller digits first.
func smallestFactorization(num int) int {
	if num < 10 {
		return num
	}
	var digits []int
	for factor := 9; factor >= 2; factor-- {
		for num%factor == 0 {
			digits = append(digits, factor)
			num /= factor
		}
	}
	if num != 1 {
		return 0
	}
	value := 0
	for i := len(digits) - 1; i >= 0; i-- {
		if value > (math.MaxInt32-digits[i])/10 {
			return 0
		}
		value = value*10 + digits[i]
	}
	return value
}
