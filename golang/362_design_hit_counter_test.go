package leetcode

import "testing"

func TestHitCounterExample(t *testing.T) {
	counter := NewHitCounter()
	counter.Hit(1)
	counter.Hit(2)
	counter.Hit(3)
	if got := counter.GetHits(4); got != 3 {
		t.Fatalf("GetHits(4) = %d, want 3", got)
	}
	counter.Hit(300)
	if got := counter.GetHits(300); got != 4 {
		t.Fatalf("GetHits(300) = %d, want 4", got)
	}
	if got := counter.GetHits(301); got != 3 {
		t.Fatalf("GetHits(301) = %d, want 3", got)
	}
}
