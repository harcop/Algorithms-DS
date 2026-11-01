package leetcode

// LeetCode #321 - Create Maximum Number
//
// Drop digits to form the largest subsequences, then merge the pair that reads largest.
func maxNumber(nums1 []int, nums2 []int, k int) []int {
	n1, n2 := len(nums1), len(nums2)
	best := make([]int, k)
	lo := 0
	if k > n2 {
		lo = k - n2
	}
	hi := n1
	if k < hi {
		hi = k
	}
	for i := lo; i <= hi; i++ {
		j := k - i
		if j > n2 {
			continue
		}
		merged := mergeMaxDigits(pickMaxSubsequence(nums1, i), pickMaxSubsequence(nums2, j))
		if intsGreater(merged, best) {
			best = merged
		}
	}
	return best
}

func pickMaxSubsequence(nums []int, k int) []int {
	if k == 0 {
		return nil
	}
	drop := len(nums) - k
	stack := make([]int, 0, k)
	for _, value := range nums {
		for drop > 0 && len(stack) > 0 && stack[len(stack)-1] < value {
			stack = stack[:len(stack)-1]
			drop--
		}
		stack = append(stack, value)
	}
	return stack[:k]
}

func mergeMaxDigits(a, b []int) []int {
	out := make([]int, 0, len(a)+len(b))
	i, j := 0, 0
	for i < len(a) || j < len(b) {
		if greaterTail(a, i, b, j) {
			out = append(out, a[i])
			i++
		} else {
			out = append(out, b[j])
			j++
		}
	}
	return out
}

func greaterTail(a []int, i int, b []int, j int) bool {
	for i < len(a) && j < len(b) {
		if a[i] != b[j] {
			return a[i] > b[j]
		}
		i++
		j++
	}
	return i < len(a)
}

func intsGreater(a, b []int) bool {
	n := len(a)
	if len(b) < n {
		n = len(b)
	}
	for i := 0; i < n; i++ {
		if a[i] != b[i] {
			return a[i] > b[i]
		}
	}
	return len(a) > len(b)
}
