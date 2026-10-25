package leetcode

// LeetCode #232 - Implement Queue using Stacks
//
// Push onto the input stack and pour it onto the output stack when the front is needed.
type MyQueue struct {
	in  []int
	out []int
}

func NewMyQueue() MyQueue {
	return MyQueue{}
}

func (queue *MyQueue) Push(x int) {
	queue.in = append(queue.in, x)
}

func (queue *MyQueue) pour() {
	if len(queue.out) > 0 {
		return
	}
	for i := len(queue.in) - 1; i >= 0; i-- {
		queue.out = append(queue.out, queue.in[i])
	}
	queue.in = queue.in[:0]
}

func (queue *MyQueue) Pop() int {
	queue.pour()
	last := len(queue.out) - 1
	value := queue.out[last]
	queue.out = queue.out[:last]
	return value
}

func (queue *MyQueue) Peek() int {
	queue.pour()
	return queue.out[len(queue.out)-1]
}

func (queue *MyQueue) Empty() bool {
	return len(queue.in) == 0 && len(queue.out) == 0
}
