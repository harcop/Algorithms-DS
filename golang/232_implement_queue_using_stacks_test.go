package leetcode

import "testing"

func TestMyQueueExample(t *testing.T) {
	queue := NewMyQueue()
	queue.Push(1)
	queue.Push(2)
	if got := queue.Peek(); got != 1 {
		t.Fatalf("Peek() = %d, want 1", got)
	}
	if got := queue.Pop(); got != 1 {
		t.Fatalf("Pop() = %d, want 1", got)
	}
	if queue.Empty() {
		t.Fatal("Empty() = true, want false")
	}
}
