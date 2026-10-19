package leetcode

// LeetCode #155 - Min Stack
//
// Each entry stores its value and the minimum at or below it.
type MinStack struct {
	stack [][2]int
}

func NewMinStack() MinStack {
	return MinStack{}
}

func (stack *MinStack) Push(val int) {
	minVal := val
	if n := len(stack.stack); n > 0 && stack.stack[n-1][1] < minVal {
		minVal = stack.stack[n-1][1]
	}
	stack.stack = append(stack.stack, [2]int{val, minVal})
}

func (stack *MinStack) Pop() {
	stack.stack = stack.stack[:len(stack.stack)-1]
}

func (stack *MinStack) Top() int {
	return stack.stack[len(stack.stack)-1][0]
}

func (stack *MinStack) GetMin() int {
	return stack.stack[len(stack.stack)-1][1]
}
