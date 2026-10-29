package leetcode

// LeetCode #292 - Nim Game
//
// The player to move wins unless the pile size is a multiple of 4.
func canWinNim(n int) bool {
	return n%4 != 0
}
