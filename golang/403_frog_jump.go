package leetcode

// LeetCode #403 - Frog Jump
//
// From a jump of size k the next jump is k-1, k, or k+1, and it must land on a stone.
func canCross(stones []int) bool {
	if len(stones) == 0 {
		return false
	}
	has := make(map[int]bool, len(stones))
	for _, stone := range stones {
		has[stone] = true
	}
	type jumpState struct{ pos, jump int }
	memo := map[jumpState]bool{}
	var reach func(pos, jump int) bool
	reach = func(pos, jump int) bool {
		if pos == stones[len(stones)-1] {
			return true
		}
		state := jumpState{pos, jump}
		if seen, ok := memo[state]; ok {
			return seen
		}
		for _, nextJump := range []int{jump - 1, jump, jump + 1} {
			if nextJump <= 0 {
				continue
			}
			next := pos + nextJump
			if has[next] && reach(next, nextJump) {
				memo[state] = true
				return true
			}
		}
		memo[state] = false
		return false
	}
	return reach(stones[0], 0)
}
