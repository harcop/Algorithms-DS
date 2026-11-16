package leetcode

import "strconv"

// LeetCode #544 - Output Contest Matches
//
// Pair the strongest remaining team with the weakest until one bracket remains.
func findContestMatch(n int) string {
	teams := make([]string, n)
	for i := range teams {
		teams[i] = strconv.Itoa(i + 1)
	}
	for len(teams) > 1 {
		next := make([]string, len(teams)/2)
		for i := range next {
			next[i] = "(" + teams[i] + "," + teams[len(teams)-1-i] + ")"
		}
		teams = next
	}
	return teams[0]
}
