package leetcode

import "strconv"

// LeetCode #553 - Optimal Division
//
// The largest value keeps the first number on top and divides it by the rest grouped together.
func optimalDivision(nums []int) string {
	if len(nums) == 1 {
		return strconv.Itoa(nums[0])
	}
	if len(nums) == 2 {
		return strconv.Itoa(nums[0]) + "/" + strconv.Itoa(nums[1])
	}
	parts := make([]string, len(nums)-1)
	for i, value := range nums[1:] {
		parts[i] = strconv.Itoa(value)
	}
	return strconv.Itoa(nums[0]) + "/(" + joinSlash(parts) + ")"
}

func joinSlash(parts []string) string {
	out := parts[0]
	for _, part := range parts[1:] {
		out += "/" + part
	}
	return out
}
