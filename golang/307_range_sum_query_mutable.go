package leetcode

// LeetCode #307 - Range Sum Query - Mutable
//
// LeetCode names this NumArray; that name already belongs to the immutable prefix array (#303).
// Store the values and add the requested slice after each update.
type NumArrayII struct {
	nums []int
}

func NewNumArrayII(nums []int) NumArrayII {
	return NumArrayII{nums: append([]int(nil), nums...)}
}

func (array *NumArrayII) Update(index int, val int) {
	array.nums[index] = val
}

func (array *NumArrayII) SumRange(left, right int) int {
	sum := 0
	for _, value := range array.nums[left : right+1] {
		sum += value
	}
	return sum
}
