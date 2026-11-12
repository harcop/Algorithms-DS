package leetcode

// LeetCode #517 - Super Washing Machines
//
// Each machine's surplus flows through a running balance, and the largest transfer on that path is the answer.
func findMinMoves(machines []int) int {
	sum := 0
	for _, dresses := range machines {
		sum += dresses
	}
	if sum%len(machines) != 0 {
		return -1
	}
	average := sum / len(machines)
	moves, balance := 0, 0
	for _, dresses := range machines {
		balance += dresses - average
		through := balance
		if through < 0 {
			through = -through
		}
		if through > moves {
			moves = through
		}
		if dresses-average > moves {
			moves = dresses - average
		}
	}
	return moves
}
