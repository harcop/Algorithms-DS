package leetcode

// LeetCode #440 - K-th Smallest in Lexicographical Order
//
// Count how many integers sit under the current prefix, then step sideways or descend.
func findKthNumber(n, k int) int {
	curr := 1
	k--
	for k > 0 {
		steps := lexSteps(curr, n)
		if steps <= k {
			k -= steps
			curr++
		} else {
			curr *= 10
			k--
		}
	}
	return curr
}

func lexSteps(curr, n int) int {
	steps := 0
	first := int64(curr)
	last := int64(curr) + 1
	limit := int64(n)
	for first <= limit {
		end := last
		if limit+1 < end {
			end = limit + 1
		}
		steps += int(end - first)
		first *= 10
		last *= 10
	}
	return steps
}
