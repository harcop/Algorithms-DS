package leetcode

// LeetCode #225 - Implement Stack using Queues
//
// After each push, rotate the older values behind the new one so the front is the top.
type MyStack struct {
	queue []int
}

func NewMyStack() MyStack {
	return MyStack{}
}

func (stack *MyStack) Push(x int) {
	stack.queue = append(stack.queue, x)
	for i := 0; i < len(stack.queue)-1; i++ {
		front := stack.queue[0]
		stack.queue = stack.queue[1:]
		stack.queue = append(stack.queue, front)
	}
}

func (stack *MyStack) Pop() int {
	value := stack.queue[0]
	stack.queue = stack.queue[1:]
	return value
}

func (stack *MyStack) Top() int {
	return stack.queue[0]
}

func (stack *MyStack) Empty() bool {
	return len(stack.queue) == 0
}
