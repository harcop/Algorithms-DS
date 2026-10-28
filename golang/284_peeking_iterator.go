package leetcode

// LeetCode #284 - Peeking Iterator
//
// Keep the next value available so peek can look at it without consuming it.
type PeekingIterator struct {
	nums []int
	i    int
}

func NewPeekingIterator(nums []int) PeekingIterator {
	return PeekingIterator{nums: nums}
}

func (it *PeekingIterator) Peek() int {
	return it.nums[it.i]
}

func (it *PeekingIterator) Next() int {
	value := it.nums[it.i]
	it.i++
	return value
}

func (it *PeekingIterator) HasNext() bool {
	return it.i < len(it.nums)
}
