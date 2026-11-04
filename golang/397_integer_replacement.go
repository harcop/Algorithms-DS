package leetcode

// LeetCode #397 - Integer Replacement
//
// Even numbers are halved, and an odd number tries both neighbors.
func integerReplacement(n int) int {
	memo := map[int]int{}
	var steps func(int) int
	steps = func(value int) int {
		if value == 1 {
			return 0
		}
		if cached, ok := memo[value]; ok {
			return cached
		}
		var answer int
		if value%2 == 0 {
			answer = 1 + steps(value/2)
		} else {
			down := steps(value - 1)
			up := steps(value + 1)
			answer = down
			if up < answer {
				answer = up
			}
			answer++
		}
		memo[value] = answer
		return answer
	}
	return steps(n)
}
