package leetcode

import (
	"sort"
	"strconv"
	"strings"
)

// LeetCode #179 - Largest Number
//
// Order the numbers so that concatenating a then b beats b then a.
func largestNumber(nums []int) string {
	parts := make([]string, len(nums))
	for i, value := range nums {
		parts[i] = strconv.Itoa(value)
	}
	sort.Slice(parts, func(i, j int) bool {
		return parts[i]+parts[j] > parts[j]+parts[i]
	})
	if parts[0] == "0" {
		return "0"
	}
	return strings.Join(parts, "")
}
