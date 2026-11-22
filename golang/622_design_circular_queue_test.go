package leetcode

import "testing"

func TestCircularQueueExample(t *testing.T) {
	q := NewCircularQueue(3)
	if q.Front() != -1 || !q.IsEmpty() || q.DeQueue() {
		t.Fatal("empty queue should have no front and refuse DeQueue")
	}
	if !q.EnQueue(1) || !q.EnQueue(2) || !q.EnQueue(3) || q.EnQueue(4) {
		t.Fatal("EnQueue did not fill a queue of size 3")
	}
	if got := q.Rear(); got != 3 || !q.IsFull() {
		t.Fatalf("Rear() = %d, IsFull() = %v, want 3 and true", got, q.IsFull())
	}
	if !q.DeQueue() || !q.EnQueue(4) {
		t.Fatal("DeQueue or the following EnQueue failed")
	}
	if got := q.Rear(); got != 4 {
		t.Fatalf("Rear() = %d, want 4", got)
	}
	if got := q.Front(); got != 2 {
		t.Fatalf("Front() = %d, want 2", got)
	}
}
