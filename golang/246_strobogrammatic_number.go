package leetcode

// LeetCode #246 - Strobogrammatic Number
//
// Each digit must rotate into the digit facing it: 0, 1, and 8 stay, and 6 swaps with 9.
func isStrobogrammatic(num string) bool {
	rotate := map[byte]byte{'0': '0', '1': '1', '6': '9', '8': '8', '9': '6'}
	for i, j := 0, len(num)-1; i <= j; i, j = i+1, j-1 {
		if rotate[num[i]] != num[j] {
			return false
		}
	}
	return true
}
