package leetcode

import "strconv"

// LeetCode #682 - Baseball Game
//
// Each token records a score, doubles or adds the recent ones, or cancels the last record.
func calPoints(operations []string) int {
	scores := make([]int, 0, len(operations))
	for _, op := range operations {
		switch op {
		case "+":
			n := len(scores)
			scores = append(scores, scores[n-1]+scores[n-2])
		case "D":
			scores = append(scores, scores[len(scores)-1]*2)
		case "C":
			scores = scores[:len(scores)-1]
		default:
			value, _ := strconv.Atoi(op)
			scores = append(scores, value)
		}
	}
	total := 0
	for _, score := range scores {
		total += score
	}
	return total
}
