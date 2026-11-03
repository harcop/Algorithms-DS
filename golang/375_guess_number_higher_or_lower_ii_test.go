package leetcode

import "testing"

func TestGetMoneyAmountExamples(t *testing.T) {
	checks := []struct {
		n, want int
	}{
		{1, 0},
		{2, 1},
		{3, 2},
		{4, 4},
		{10, 16},
	}
	for _, check := range checks {
		if got := getMoneyAmount(check.n); got != check.want {
			t.Fatalf("getMoneyAmount(%d) = %d, want %d", check.n, got, check.want)
		}
	}
}
