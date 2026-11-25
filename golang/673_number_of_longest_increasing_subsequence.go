package leetcode

// LeetCode #673 - Number of Longest Increasing Subsequence
//
// Each value extends the longest chains that end before it, and equal lengths add their counts.
func findNumberOfLIS(nums []int) int {
	length := make([]int, len(nums))
	count := make([]int, len(nums))
	best, total := 0, 0
	for i := range nums {
		length[i], count[i] = 1, 1
		for j := 0; j < i; j++ {
			if nums[j] >= nums[i] {
				continue
			}
			if length[j]+1 > length[i] {
				length[i] = length[j] + 1
				count[i] = count[j]
			} else if length[j]+1 == length[i] {
				count[i] += count[j]
			}
		}
		if length[i] > best {
			best = length[i]
			total = count[i]
		} else if length[i] == best {
			total += count[i]
		}
	}
	return total
}
