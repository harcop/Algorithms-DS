package leetcode

// LeetCode #416 - Partition Equal Subset Sum
//
// The array splits evenly when some subset sums to half of the total.
func canPartition(nums []int) bool {
	sum := 0
	for _, num := range nums {
		sum += num
	}
	if sum%2 != 0 {
		return false
	}
	target := sum / 2
	reach := make([]bool, target+1)
	reach[0] = true
	for _, num := range nums {
		for total := target; total >= num; total-- {
			if reach[total-num] {
				reach[total] = true
			}
		}
	}
	return reach[target]
}
