package leetcode

import (
	"strconv"
	"strings"
)

// LeetCode #636 - Exclusive Time of Functions
//
// The running call sits on a stack, and the time since the previous log belongs to it.
func exclusiveTime(n int, logs []string) []int {
	times := make([]int, n)
	var stack []int
	previous := 0
	for _, log := range logs {
		parts := strings.Split(log, ":")
		id, _ := strconv.Atoi(parts[0])
		timestamp, _ := strconv.Atoi(parts[2])
		if parts[1] == "start" {
			if len(stack) > 0 {
				times[stack[len(stack)-1]] += timestamp - previous
			}
			stack = append(stack, id)
			previous = timestamp
			continue
		}
		times[stack[len(stack)-1]] += timestamp - previous + 1
		stack = stack[:len(stack)-1]
		previous = timestamp + 1
	}
	return times
}
