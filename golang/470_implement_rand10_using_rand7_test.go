package leetcode

import "testing"

func TestRand10InRange(t *testing.T) {
	rand7Draws = []int{3, 7, 4, 6, 5, 2, 1, 7, 3, 4, 6, 5, 7, 2, 1}
	rand7At = 0
	if got := rand10(); got != 1 {
		t.Fatalf("rand10() = %d, want 1", got)
	}
	for i := 0; i < 20; i++ {
		got := rand10()
		if got < 1 || got > 10 {
			t.Fatalf("rand10() = %d, want 1 through 10", got)
		}
	}
}
