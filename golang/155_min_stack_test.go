package leetcode

import "testing"

func TestMinStackExample(t *testing.T) {
	stack := NewMinStack()
	stack.Push(-2)
	stack.Push(0)
	stack.Push(-3)
	if got := stack.GetMin(); got != -3 {
		t.Fatalf("GetMin() = %d, want -3", got)
	}
	stack.Pop()
	if got := stack.Top(); got != 0 {
		t.Fatalf("Top() = %d, want 0", got)
	}
	if got := stack.GetMin(); got != -2 {
		t.Fatalf("GetMin() = %d, want -2", got)
	}
}
