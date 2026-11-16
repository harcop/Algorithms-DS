package leetcode

// LeetCode #548 - Split Array with Equal Sum
//
// LeetCode names this splitArray; that name already belongs to the largest-sum split (#410).
// Two cuts on the left and two on the right make four parts, and the answer is whether those sums can match.
func splitArrayEqualSum(nums []int) bool {
	n := len(nums)
	if n < 7 {
		return false
	}
	prefix := make([]int64, n)
	prefix[0] = int64(nums[0])
	for i := 1; i < n; i++ {
		prefix[i] = prefix[i-1] + int64(nums[i])
	}
	for mid := 3; mid < n-3; mid++ {
		seen := map[int64]bool{}
		for left := 1; left < mid-1; left++ {
			first := prefix[left-1]
			second := prefix[mid-1] - prefix[left]
			if first == second {
				seen[first] = true
			}
		}
		for right := mid + 2; right < n-1; right++ {
			third := prefix[right-1] - prefix[mid]
			fourth := prefix[n-1] - prefix[right]
			if third == fourth && seen[third] {
				return true
			}
		}
	}
	return false
}
