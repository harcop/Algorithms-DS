package leetcode

import "testing"

func TestMyStackExample(t *testing.T) {
	stack := NewMyStack()
	stack.Push(1)
	stack.Push(2)
	if got := stack.Top(); got != 2 {
		t.Fatalf("Top() = %d, want 2", got)
	}
	if got := stack.Pop(); got != 2 {
		t.Fatalf("Pop() = %d, want 2", got)
	}
	if stack.Empty() {
		t.Fatal("Empty() = true, want false")
	}
}
