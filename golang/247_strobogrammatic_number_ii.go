package leetcode

// LeetCode #247 - Strobogrammatic Number II
//
// Build each number from the outside in, using a digit and the digit it rotates into.
func findStrobogrammatic(n int) []string {
	pairs := [][2]byte{{'0', '0'}, {'1', '1'}, {'6', '9'}, {'8', '8'}, {'9', '6'}}
	buf := make([]byte, n)
	var out []string
	var fill func(lo, hi int)
	fill = func(lo, hi int) {
		if lo > hi {
			out = append(out, string(buf))
			return
		}
		for _, pair := range pairs {
			a, b := pair[0], pair[1]
			if lo == 0 && n > 1 && a == '0' {
				continue
			}
			if lo == hi && a != '0' && a != '1' && a != '8' {
				continue
			}
			buf[lo] = a
			buf[hi] = b
			fill(lo+1, hi-1)
		}
	}
	fill(0, n-1)
	return out
}
