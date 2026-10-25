package leetcode

// LeetCode #239 - Sliding Window Maximum
//
// Keep indexes of candidates in decreasing order and drop those that leave the window.
func maxSlidingWindow(nums []int, k int) []int {
	var index []int
	var out []int
	for i, value := range nums {
		for len(index) > 0 && nums[index[len(index)-1]] <= value {
			index = index[:len(index)-1]
		}
		index = append(index, i)
		if i >= k && index[0] == i-k {
			index = index[1:]
		}
		if i+1 >= k {
			out = append(out, nums[index[0]])
		}
	}
	return out
}
