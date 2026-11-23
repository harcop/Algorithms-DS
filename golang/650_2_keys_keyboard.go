package leetcode

// LeetCode #650 - 2 Keys Keyboard
//
// The cheapest way to reach n copies is to copy once for each prime factor and paste the rest.
func minSteps(n int) int {
	steps := 0
	for factor := 2; n > 1; factor++ {
		for n%factor == 0 {
			steps += factor
			n /= factor
		}
	}
	return steps
}
