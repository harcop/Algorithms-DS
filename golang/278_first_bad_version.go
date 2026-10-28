package leetcode

// badVersionCutoff is the first bad version used by the mocked isBadVersion check.
var badVersionCutoff int

func isBadVersion(version int) bool {
	return version >= badVersionCutoff
}

// LeetCode #278 - First Bad Version
//
// Binary search for the first version the check marks as bad.
func firstBadVersion(n int) int {
	lo, hi := 1, n
	for lo < hi {
		mid := lo + (hi-lo)/2
		if isBadVersion(mid) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
