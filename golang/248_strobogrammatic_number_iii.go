package leetcode

// LeetCode #248 - Strobogrammatic Number III
//
// Count the strobogrammatic numbers of each length between low and high.
func strobogrammaticInRange(low string, high string) int {
	pairs := [][2]byte{{'0', '0'}, {'1', '1'}, {'6', '9'}, {'8', '8'}, {'9', '6'}}
	count := 0
	for length := len(low); length <= len(high); length++ {
		buf := make([]byte, length)
		var fill func(lo, hi int)
		fill = func(lo, hi int) {
			if lo > hi {
				number := string(buf)
				if (length > len(low) || number >= low) && (length < len(high) || number <= high) {
					count++
				}
				return
			}
			for _, pair := range pairs {
				a, b := pair[0], pair[1]
				if lo == 0 && length > 1 && a == '0' {
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
		fill(0, length-1)
	}
	return count
}
