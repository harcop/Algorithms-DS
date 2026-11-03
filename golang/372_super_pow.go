package leetcode

// LeetCode #372 - Super Pow
//
// Raise the base one exponent digit at a time, keeping every power mod 1337.
func superPow(a int, b []int) int {
	const mod = 1337
	result := 1
	for _, digit := range b {
		result = powMod(result, 10, mod) * powMod(a, digit, mod) % mod
	}
	return result
}

func powMod(base, exp, mod int) int {
	result := 1
	base %= mod
	for exp > 0 {
		if exp%2 == 1 {
			result = result * base % mod
		}
		base = base * base % mod
		exp /= 2
	}
	return result
}
