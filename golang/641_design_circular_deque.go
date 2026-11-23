package leetcode

// LeetCode #641 - Design Circular Deque
//
// A fixed buffer stores the values, and the head index steps backward when a value is inserted at the front.
type MyCircularDeque struct {
	data  []int
	head  int
	count int
}

func NewCircularDeque(k int) *MyCircularDeque {
	return &MyCircularDeque{data: make([]int, k)}
}

func (q *MyCircularDeque) InsertFront(value int) bool {
	if q.IsFull() {
		return false
	}
	q.head = (q.head - 1 + len(q.data)) % len(q.data)
	q.data[q.head] = value
	q.count++
	return true
}

func (q *MyCircularDeque) InsertLast(value int) bool {
	if q.IsFull() {
		return false
	}
	q.data[(q.head+q.count)%len(q.data)] = value
	q.count++
	return true
}

func (q *MyCircularDeque) DeleteFront() bool {
	if q.IsEmpty() {
		return false
	}
	q.head = (q.head + 1) % len(q.data)
	q.count--
	return true
}

func (q *MyCircularDeque) DeleteLast() bool {
	if q.IsEmpty() {
		return false
	}
	q.count--
	return true
}

func (q *MyCircularDeque) GetFront() int {
	if q.IsEmpty() {
		return -1
	}
	return q.data[q.head]
}

func (q *MyCircularDeque) GetRear() int {
	if q.IsEmpty() {
		return -1
	}
	return q.data[(q.head+q.count-1)%len(q.data)]
}

func (q *MyCircularDeque) IsEmpty() bool { return q.count == 0 }

func (q *MyCircularDeque) IsFull() bool { return q.count == len(q.data) }
