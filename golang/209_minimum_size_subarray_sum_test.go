package leetcode

import "testing"

func TestMinSubArrayLenExampleOne(t *testing.T) {
	if got := minSubArrayLen(7, []int{2, 3, 1, 2, 4, 3}); got != 2 {
		t.Fatalf("minSubArrayLen() = %d, want 2", got)
	}
}

func TestMinSubArrayLenExampleTwo(t *testing.T) {
	if got := minSubArrayLen(4, []int{1, 4, 4}); got != 1 {
		t.Fatalf("minSubArrayLen() = %d, want 1", got)
	}
}

func TestMinSubArrayLenExampleThree(t *testing.T) {
	if got := minSubArrayLen(11, []int{1, 1, 1, 1, 1, 1, 1, 1}); got != 0 {
		t.Fatalf("minSubArrayLen() = %d, want 0", got)
	}
}
