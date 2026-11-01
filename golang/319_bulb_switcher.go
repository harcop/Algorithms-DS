package leetcode

import "math"

// LeetCode #319 - Bulb Switcher
//
// A bulb stays on only when it is a perfect square, so the answer is floor of sqrt(n).
func bulbSwitch(n int) int {
	return int(math.Sqrt(float64(n)))
}
