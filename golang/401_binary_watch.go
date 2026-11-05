package leetcode

import (
	"fmt"
	"math/bits"
)

// LeetCode #401 - Binary Watch
//
// A valid time is every hour and minute whose set bits add up to the given count.
func readBinaryWatch(turnedOn int) []string {
	var out []string
	for hour := 0; hour < 12; hour++ {
		for minute := 0; minute < 60; minute++ {
			bitsOn := bits.OnesCount(uint(hour)) + bits.OnesCount(uint(minute))
			if bitsOn == turnedOn {
				out = append(out, fmt.Sprintf("%d:%02d", hour, minute))
			}
		}
	}
	return out
}
