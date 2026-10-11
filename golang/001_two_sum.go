package leetcode

// LeetCode #1 - Two Sum
//
// Returns the indices of the two numbers such that they add up to target.
// Assumes exactly one valid answer exists.
func twoSum(nums []int, target int) []int {
	seen := make(map[int]int)

	for index, value := range nums {
		complement := target - value
		if prev, ok := seen[complement]; ok {
			return []int{prev, index}
		}
		seen[value] = index
	}

	return nil
}
