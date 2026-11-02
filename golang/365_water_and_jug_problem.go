package leetcode

// LeetCode #365 - Water and Jug Problem
//
// The two jugs can measure any multiple of their greatest common divisor, up to their combined size.
func canMeasureWater(x, y, target int) bool {
	if target == 0 {
		return true
	}
	if target > x+y {
		return false
	}
	if x == 0 || y == 0 {
		return target == x+y
	}
	a, b := x, y
	for b != 0 {
		a, b = b, a%b
	}
	return target%a == 0
}
