package leetcode

// guessTarget is the hidden number used by guess.
var guessTarget int

func guess(num int) int {
	if guessTarget < num {
		return -1
	}
	if guessTarget > num {
		return 1
	}
	return 0
}

// LeetCode #374 - Guess Number Higher or Lower
//
// Binary search moves up when the guess is low and down when it is high.
func guessNumber(n int) int {
	lo, hi := 1, n
	for lo < hi {
		mid := lo + (hi-lo)/2
		switch guess(mid) {
		case 0:
			return mid
		case 1:
			lo = mid + 1
		default:
			hi = mid - 1
		}
	}
	return lo
}
