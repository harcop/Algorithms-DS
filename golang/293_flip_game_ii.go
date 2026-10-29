package leetcode

// LeetCode #293 - Flip Game II
//
// A move wins when it leaves the opponent with no winning reply.
func canWin(currentState string) bool {
	memo := map[string]bool{}
	var win func([]byte) bool
	win = func(state []byte) bool {
		key := string(state)
		if known, ok := memo[key]; ok {
			return known
		}
		for i := 0; i+1 < len(state); i++ {
			if state[i] != '+' || state[i+1] != '+' {
				continue
			}
			state[i], state[i+1] = '-', '-'
			opponentWins := win(state)
			state[i], state[i+1] = '+', '+'
			if !opponentWins {
				memo[key] = true
				return true
			}
		}
		memo[key] = false
		return false
	}
	return win([]byte(currentState))
}
