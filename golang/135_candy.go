package leetcode

// LeetCode #135 - Candy
//
// Give each child the stricter of the increasing run from the left and the increasing run from the right.
func candy(ratings []int) int {
	n := len(ratings)
	left := make([]int, n)
	right := make([]int, n)
	for i := range ratings {
		left[i], right[i] = 1, 1
	}
	for i := 1; i < n; i++ {
		if ratings[i] > ratings[i-1] {
			left[i] = left[i-1] + 1
		}
	}
	for i := n - 2; i >= 0; i-- {
		if ratings[i] > ratings[i+1] {
			right[i] = right[i+1] + 1
		}
	}
	total := 0
	for i := range ratings {
		if left[i] > right[i] {
			total += left[i]
		} else {
			total += right[i]
		}
	}
	return total
}
