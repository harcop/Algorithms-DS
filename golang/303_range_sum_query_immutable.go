package leetcode

// LeetCode #303 - Range Sum Query - Immutable
//
// A prefix sum answers any range as the difference of two prefixes.
type NumArray struct {
	prefix []int
}

func NewNumArray(nums []int) NumArray {
	prefix := make([]int, len(nums)+1)
	for i, value := range nums {
		prefix[i+1] = prefix[i] + value
	}
	return NumArray{prefix: prefix}
}

func (array NumArray) SumRange(left, right int) int {
	return array.prefix[right+1] - array.prefix[left]
}
