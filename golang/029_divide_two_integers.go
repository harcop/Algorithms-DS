package leetcode

import "math"

// LeetCode #29 - Divide Two Integers
//
// Divide without using multiplication, division, or modulo. Repeatedly subtract the
// largest doubled multiple of the divisor that still fits.
func divide(dividend int, divisor int) int {
	if dividend == math.MinInt32 && divisor == -1 {
		return math.MaxInt32
	}

	negative := (dividend < 0) != (divisor < 0)
	a := int64(dividend)
	b := int64(divisor)
	if a < 0 {
		a = -a
	}
	if b < 0 {
		b = -b
	}

	var result int64
	for a >= b {
		temp := b
		multiple := int64(1)
		for a >= (temp << 1) {
			temp <<= 1
			multiple <<= 1
		}
		a -= temp
		result += multiple
	}

	if negative {
		result = -result
	}
	return int(result)
}
