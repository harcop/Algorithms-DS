package leetcode

import "testing"

func TestMedianFinderExample(t *testing.T) {
	finder := NewMedianFinder()
	finder.AddNum(1)
	finder.AddNum(2)
	if got := finder.FindMedian(); got != 1.5 {
		t.Fatalf("FindMedian() = %v, want 1.5", got)
	}
	finder.AddNum(3)
	if got := finder.FindMedian(); got != 2 {
		t.Fatalf("FindMedian() = %v, want 2", got)
	}
}
