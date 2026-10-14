package leetcode

// LeetCode #50 - Pow(x, n)
//
// Square the base while halving the exponent. A negative exponent inverts the result.
func myPow(x float64, n int) float64 {
	exp := int64(n)
	if exp < 0 {
		x = 1 / x
		exp = -exp
	}

	result := 1.0
	for exp > 0 {
		if exp%2 == 1 {
			result *= x
		}
		x *= x
		exp /= 2
	}
	return result
}
