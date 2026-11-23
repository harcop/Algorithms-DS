package leetcode

import "testing"

func TestCircularDequeExample(t *testing.T) {
	q := NewCircularDeque(3)
	if q.GetFront() != -1 || !q.IsEmpty() || q.DeleteFront() {
		t.Fatal("empty deque should have no front and refuse DeleteFront")
	}
	if !q.InsertLast(1) || !q.InsertLast(2) || !q.InsertFront(3) || q.InsertFront(4) {
		t.Fatal("InsertFront or InsertLast did not fill a deque of size 3")
	}
	if got := q.GetRear(); got != 2 || !q.IsFull() {
		t.Fatalf("GetRear() = %d, IsFull() = %v, want 2 and true", got, q.IsFull())
	}
	if !q.DeleteLast() || !q.InsertFront(4) {
		t.Fatal("DeleteLast or the following InsertFront failed")
	}
	if got := q.GetFront(); got != 4 {
		t.Fatalf("GetFront() = %d, want 4", got)
	}
	if got := q.GetRear(); got != 1 {
		t.Fatalf("GetRear() = %d, want 1", got)
	}
}
