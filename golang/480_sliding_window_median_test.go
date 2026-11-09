package leetcode

import "testing"

func TestMedianSlidingWindowExamples(t *testing.T) {
	got := medianSlidingWindow([]int{1, 3, -1, -3, 5, 3, 6, 7}, 3)
	want := []float64{1, -1, -1, 3, 5, 6}
	if !closeFloats(got, want) {
		t.Fatalf("medianSlidingWindow() = %v, want %v", got, want)
	}
	got = medianSlidingWindow([]int{1, 2, 3, 4, 2, 3, 1, 4, 2}, 3)
	want = []float64{2, 3, 3, 3, 2, 3, 2}
	if !closeFloats(got, want) {
		t.Fatalf("medianSlidingWindow() = %v, want %v", got, want)
	}
}

func closeFloats(a, b []float64) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		diff := a[i] - b[i]
		if diff < -1e-5 || diff > 1e-5 {
			return false
		}
	}
	return true
}
