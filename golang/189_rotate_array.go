package leetcode

// LeetCode #189 - Rotate Array
//
// LeetCode names this rotate; that name already belongs to the matrix rotation (#48).
// Reverse the whole slice, then reverse the two pieces around the rotation point.
func rotateArray(nums []int, k int) {
	n := len(nums)
	if n == 0 {
		return
	}
	k %= n
	if k == 0 {
		return
	}
	reverse := func(i, j int) {
		for i < j {
			nums[i], nums[j] = nums[j], nums[i]
			i++
			j--
		}
	}
	reverse(0, n-1)
	reverse(0, k-1)
	reverse(k, n-1)
}
