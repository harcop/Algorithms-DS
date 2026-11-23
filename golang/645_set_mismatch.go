package leetcode

// LeetCode #645 - Set Mismatch
//
// The repeated value is seen twice, and the missing value is the gap between the expected sum and the rest.
func findErrorNums(nums []int) []int {
	seen := make([]bool, len(nums)+1)
	duplicate := 0
	sum := 0
	for _, num := range nums {
		if seen[num] {
			duplicate = num
		}
		seen[num] = true
		sum += num
	}
	expected := len(nums) * (len(nums) + 1) / 2
	return []int{duplicate, expected - (sum - duplicate)}
}
