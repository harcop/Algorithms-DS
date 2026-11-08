package leetcode

// LeetCode #448 - Find All Numbers Disappeared in an Array
//
// Negate the slot for each value that appears, then the positive slots are the missing numbers.
func findDisappearedNumbers(nums []int) []int {
	for _, value := range nums {
		index := absInt(value) - 1
		if nums[index] > 0 {
			nums[index] = -nums[index]
		}
	}
	var missing []int
	for i, value := range nums {
		if value > 0 {
			missing = append(missing, i+1)
		}
	}
	return missing
}
