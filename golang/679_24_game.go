package leetcode

import "math"

// LeetCode #679 - 24 Game
//
// Every pair is reduced by +, -, *, or /, and the last remaining value has to land on 24.
func judgePoint24(cards []int) bool {
	nums := make([]float64, len(cards))
	for i, card := range cards {
		nums[i] = float64(card)
	}
	return canReach24(nums)
}

func canReach24(nums []float64) bool {
	if len(nums) == 1 {
		return math.Abs(nums[0]-24) < 1e-6
	}
	for i := 0; i < len(nums); i++ {
		for j := 0; j < len(nums); j++ {
			if i == j {
				continue
			}
			rest := make([]float64, 0, len(nums)-1)
			for k, value := range nums {
				if k != i && k != j {
					rest = append(rest, value)
				}
			}
			for _, value := range operate24(nums[i], nums[j]) {
				rest = append(rest, value)
				if canReach24(rest) {
					return true
				}
				rest = rest[:len(rest)-1]
			}
		}
	}
	return false
}

func operate24(a, b float64) []float64 {
	values := []float64{a + b, a - b, a * b}
	if math.Abs(b) > 1e-6 {
		values = append(values, a/b)
	}
	return values
}
