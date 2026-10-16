package leetcode

// LeetCode #75 - Sort Colors
//
// Dutch-flag partition: zeros grow from the left, twos shrink from the right, ones stay in the middle.
func sortColors(nums []int) {
	low, mid, high := 0, 0, len(nums)
	for mid < high {
		switch nums[mid] {
		case 0:
			nums[low], nums[mid] = nums[mid], nums[low]
			low++
			mid++
		case 1:
			mid++
		default:
			high--
			nums[mid], nums[high] = nums[high], nums[mid]
		}
	}
}
