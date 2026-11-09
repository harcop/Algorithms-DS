package leetcode

// LeetCode #464 - Can I Win
//
// A move wins when it reaches the total or leaves the opponent with no winning reply, and each subset of used numbers is memoized.
func canIWin(maxChoosableInteger int, desiredTotal int) bool {
	if desiredTotal <= 0 {
		return true
	}
	if desiredTotal <= maxChoosableInteger {
		return true
	}
	sum := maxChoosableInteger * (maxChoosableInteger + 1) / 2
	if sum < desiredTotal {
		return false
	}
	memo := map[uint32]bool{}
	var play func(mask uint32, total int) bool
	play = func(mask uint32, total int) bool {
		if seen, ok := memo[mask]; ok {
			return seen
		}
		for i := 1; i <= maxChoosableInteger; i++ {
			bit := uint32(1) << (i - 1)
			if mask&bit != 0 {
				continue
			}
			if total+i >= desiredTotal || !play(mask|bit, total+i) {
				memo[mask] = true
				return true
			}
		}
		memo[mask] = false
		return false
	}
	return play(0, 0)
}
