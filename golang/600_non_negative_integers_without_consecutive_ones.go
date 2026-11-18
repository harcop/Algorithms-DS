package leetcode

// LeetCode #600 - Non-negative Integers without Consecutive Ones
//
// Walk the bits of n and skip any placement that would put two ones in a row.
func findIntegers(n int) int {
	if n == 0 {
		return 1
	}
	var bits []int
	for value := n; value > 0; value >>= 1 {
		bits = append(bits, value&1)
	}
	for i, j := 0, len(bits)-1; i < j; i, j = i+1, j-1 {
		bits[i], bits[j] = bits[j], bits[i]
	}
	memo := make([][2][2]int, len(bits))
	for i := range memo {
		memo[i] = [2][2]int{{-1, -1}, {-1, -1}}
	}
	var count func(index, previous, tight int) int
	count = func(index, previous, tight int) int {
		if index == len(bits) {
			return 1
		}
		if memo[index][previous][tight] != -1 {
			return memo[index][previous][tight]
		}
		limit := 1
		if tight == 1 {
			limit = bits[index]
		}
		total := 0
		for digit := 0; digit <= limit; digit++ {
			if previous == 1 && digit == 1 {
				continue
			}
			nextTight := 0
			if tight == 1 && digit == limit {
				nextTight = 1
			}
			total += count(index+1, digit, nextTight)
		}
		memo[index][previous][tight] = total
		return total
	}
	return count(0, 0, 1)
}
