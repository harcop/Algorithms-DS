package leetcode

// LeetCode #281 - Zigzag Iterator
//
// Alternate between the two lists, skipping a list once it is exhausted.
type ZigzagIterator struct {
	lists [2][]int
	index [2]int
	turn  int
}

func NewZigzagIterator(v1, v2 []int) ZigzagIterator {
	return ZigzagIterator{lists: [2][]int{v1, v2}}
}

func (it *ZigzagIterator) Next() int {
	for step := 0; step < 2; step++ {
		side := it.turn
		if it.index[side] < len(it.lists[side]) {
			value := it.lists[side][it.index[side]]
			it.index[side]++
			it.turn ^= 1
			return value
		}
		it.turn ^= 1
	}
	return 0
}

func (it *ZigzagIterator) HasNext() bool {
	return it.index[0] < len(it.lists[0]) || it.index[1] < len(it.lists[1])
}
