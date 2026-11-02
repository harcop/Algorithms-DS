package leetcode

// LeetCode #360 - Sort Transformed Array
//
// A parabola is sorted by taking the larger or smaller end, depending on which way it opens.
func sortTransformedArray(nums []int, a, b, c int) []int {
	n := len(nums)
	out := make([]int, n)
	value := func(x int) int { return a*x*x + b*x + c }
	i, j := 0, n-1
	if a >= 0 {
		for index := n - 1; i <= j; index-- {
			left, right := value(nums[i]), value(nums[j])
			if left > right {
				out[index] = left
				i++
			} else {
				out[index] = right
				j--
			}
		}
		return out
	}
	for index := 0; i <= j; index++ {
		left, right := value(nums[i]), value(nums[j])
		if left < right {
			out[index] = left
			i++
		} else {
			out[index] = right
			j--
		}
	}
	return out
}
