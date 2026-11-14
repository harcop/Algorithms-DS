package leetcode

import "testing"

func TestWeightedPickerExamples(t *testing.T) {
	single := NewWeightedPicker([]int{1})
	if got := single.PickIndex(); got != 0 {
		t.Fatalf("PickIndex() = %d, want 0", got)
	}
	picker := NewWeightedPicker([]int{1, 3})
	for i := 0; i < 50; i++ {
		got := picker.PickIndex()
		if got != 0 && got != 1 {
			t.Fatalf("PickIndex() = %d, want 0 or 1", got)
		}
	}
}
