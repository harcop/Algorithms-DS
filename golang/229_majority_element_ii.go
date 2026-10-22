package leetcode

import "sort"

// LeetCode #229 - Majority Element II
//
// LeetCode names this majorityElement; that name already belongs to the single majority (#169).
// At most two values can appear more than n/3 times, so Boyer–Moore keeps two candidates.
func majorityElementII(nums []int) []int {
	a, b := 0, 1
	countA, countB := 0, 0
	for _, value := range nums {
		switch {
		case value == a:
			countA++
		case value == b:
			countB++
		case countA == 0:
			a, countA = value, 1
		case countB == 0:
			b, countB = value, 1
		default:
			countA--
			countB--
		}
	}
	countA, countB = 0, 0
	for _, value := range nums {
		if value == a {
			countA++
		} else if value == b {
			countB++
		}
	}
	var out []int
	if countA > len(nums)/3 {
		out = append(out, a)
	}
	if a != b && countB > len(nums)/3 {
		out = append(out, b)
	}
	sort.Ints(out)
	return out
}
