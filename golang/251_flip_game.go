package leetcode

// LeetCode #251 - Flip Game
//
// Each pair of plus signs can be flipped to a pair of minus signs.
func generatePossibleNextMoves(currentState string) []string {
	var out []string
	for i := 0; i+1 < len(currentState); i++ {
		if currentState[i] == '+' && currentState[i+1] == '+' {
			out = append(out, currentState[:i]+"--"+currentState[i+2:])
		}
	}
	return out
}
