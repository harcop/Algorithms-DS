package leetcode

import "strconv"

// LeetCode #670 - Maximum Swap
//
// Swap the first digit with the largest later digit that can take its place.
func maximumSwap(num int) int {
	digits := []byte(strconv.Itoa(num))
	last := [10]int{}
	for i := range last {
		last[i] = -1
	}
	for i, digit := range digits {
		last[digit-'0'] = i
	}
	for i, digit := range digits {
		for d := 9; d > int(digit-'0'); d-- {
			if last[d] <= i {
				continue
			}
			digits[i], digits[last[d]] = digits[last[d]], digits[i]
			swapped, _ := strconv.Atoi(string(digits))
			return swapped
		}
	}
	return num
}
