package leetcode

// LeetCode #233 - Number of Digit One
//
// Count how often each digit place contributes a 1.
func countDigitOne(n int) int {
	ans := int64(0)
	value := int64(n)
	for place := int64(1); place <= value; place *= 10 {
		divider := place * 10
		extra := value%divider - place + 1
		if extra < 0 {
			extra = 0
		} else if extra > place {
			extra = place
		}
		ans += value/divider*place + extra
	}
	return int(ans)
}
