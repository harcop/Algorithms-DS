package leetcode

import "sort"

// LeetCode #473 - Matchsticks to Square
//
// The sticks form a square when they can be packed into four sides of equal length.
func makesquare(matchsticks []int) bool {
	sum := int64(0)
	for _, stick := range matchsticks {
		sum += int64(stick)
	}
	if sum%4 != 0 {
		return false
	}
	side := int(sum / 4)
	sticks := append([]int(nil), matchsticks...)
	sort.Slice(sticks, func(i, j int) bool { return sticks[i] > sticks[j] })
	sides := [4]int{}
	var pack func(index int) bool
	pack = func(index int) bool {
		if index == len(sticks) {
			return sides[0] == side && sides[1] == side && sides[2] == side && sides[3] == side
		}
		for k := 0; k < 4; k++ {
			if sides[k]+sticks[index] > side {
				continue
			}
			sides[k] += sticks[index]
			if pack(index + 1) {
				return true
			}
			sides[k] -= sticks[index]
			if sides[k] == 0 {
				break
			}
		}
		return false
	}
	return pack(0)
}
