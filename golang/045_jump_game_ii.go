package leetcode

// LeetCode #45 - Jump Game II
//
// Within the current jump's reach, track the farthest index the next jump can cover.
func jump(nums []int) int {
	if len(nums) <= 1 {
		return 0
	}

	jumps := 0
	currentEnd := 0
	farthest := 0

	for i := 0; i < len(nums)-1; i++ {
		if next := i + nums[i]; next > farthest {
			farthest = next
		}
		if i == currentEnd {
			jumps++
			currentEnd = farthest
		}
	}

	return jumps
}
