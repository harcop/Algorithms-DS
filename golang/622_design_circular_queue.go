package leetcode

// LeetCode #622 - Design Circular Queue
//
// A fixed buffer stores the values, and the head index wraps around when it reaches the end.
type MyCircularQueue struct {
	data  []int
	head  int
	count int
}

func NewCircularQueue(k int) *MyCircularQueue {
	return &MyCircularQueue{data: make([]int, k)}
}

func (q *MyCircularQueue) EnQueue(value int) bool {
	if q.IsFull() {
		return false
	}
	q.data[(q.head+q.count)%len(q.data)] = value
	q.count++
	return true
}

func (q *MyCircularQueue) DeQueue() bool {
	if q.IsEmpty() {
		return false
	}
	q.head = (q.head + 1) % len(q.data)
	q.count--
	return true
}

func (q *MyCircularQueue) Front() int {
	if q.IsEmpty() {
		return -1
	}
	return q.data[q.head]
}

func (q *MyCircularQueue) Rear() int {
	if q.IsEmpty() {
		return -1
	}
	return q.data[(q.head+q.count-1)%len(q.data)]
}

func (q *MyCircularQueue) IsEmpty() bool { return q.count == 0 }

func (q *MyCircularQueue) IsFull() bool { return q.count == len(q.data) }
