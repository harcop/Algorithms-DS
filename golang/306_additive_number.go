package leetcode

import "strconv"

// LeetCode #306 - Additive Number
//
// Split the string into the first two numbers and check that the rest is their running sum.
func isAdditiveNumber(num string) bool {
	n := len(num)
	for i := 1; i <= n/2; i++ {
		if num[0] == '0' && i > 1 {
			continue
		}
		for j := i + 1; j < n; j++ {
			if num[i] == '0' && j > i+1 {
				continue
			}
			a, _ := strconv.ParseInt(num[:i], 10, 64)
			b, _ := strconv.ParseInt(num[i:j], 10, 64)
			k := j
			ok := true
			for k < n {
				sum := a + b
				text := strconv.FormatInt(sum, 10)
				if k+len(text) > n || num[k:k+len(text)] != text {
					ok = false
					break
				}
				k += len(text)
				a, b = b, sum
			}
			if ok && k == n {
				return true
			}
		}
	}
	return false
}
