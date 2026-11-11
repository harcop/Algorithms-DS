package leetcode

// LeetCode #494 - Target Sum
//
// Assigning signs is the same as choosing a subset that sums to half of the total plus the target.
func findTargetSumWays(nums []int, target int) int {
	sum := 0
	for _, value := range nums {
		sum += value
	}
	if target > sum || target < -sum || (sum+target)%2 != 0 {
		return 0
	}
	want := (sum + target) / 2
	ways := make([]int, want+1)
	ways[0] = 1
	for _, value := range nums {
		for total := want; total >= value; total-- {
			ways[total] += ways[total-value]
		}
	}
	return ways[want]
}
