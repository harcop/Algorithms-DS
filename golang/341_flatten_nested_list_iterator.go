package leetcode

// LeetCode #341 - Flatten Nested List Iterator
//
// A stack of lists yields the next integer and pushes a nested list when one is reached.
type NestedIterator struct {
	stack   []nestedFrame
	nextVal int
	has     bool
}

type nestedFrame struct {
	items []NestedInteger
	index int
}

func NewNestedIterator(nested []NestedInteger) *NestedIterator {
	it := &NestedIterator{stack: []nestedFrame{{items: nested}}}
	it.advance()
	return it
}

func (it *NestedIterator) advance() {
	it.has = false
	for len(it.stack) > 0 {
		top := len(it.stack) - 1
		frame := it.stack[top]
		if frame.index >= len(frame.items) {
			it.stack = it.stack[:top]
			continue
		}
		item := frame.items[frame.index]
		it.stack[top].index++
		if item.value != nil {
			it.nextVal = *item.value
			it.has = true
			return
		}
		it.stack = append(it.stack, nestedFrame{items: item.list})
	}
}

func (it *NestedIterator) Next() int {
	value := it.nextVal
	it.advance()
	return value
}

func (it *NestedIterator) HasNext() bool {
	return it.has
}
