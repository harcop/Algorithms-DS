package leetcode

import "testing"

func TestMapSumExample(t *testing.T) {
	ms := NewMapSum()
	ms.Insert("apple", 3)
	if got := ms.Sum("ap"); got != 3 {
		t.Fatalf("Sum(ap) = %d, want 3", got)
	}
	ms.Insert("app", 2)
	if got := ms.Sum("ap"); got != 5 {
		t.Fatalf("Sum(ap) = %d, want 5", got)
	}
	ms.Insert("apple", 2)
	if got := ms.Sum("ap"); got != 4 {
		t.Fatalf("Sum(ap) = %d, want 4", got)
	}
}
