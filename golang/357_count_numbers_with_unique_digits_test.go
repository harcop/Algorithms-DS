package leetcode

import "testing"

func TestCountNumbersWithUniqueDigits(t *testing.T) {
	checks := []struct {
		n, want int
	}{
		{0, 1},
		{1, 10},
		{2, 91},
		{3, 739},
	}
	for _, check := range checks {
		if got := countNumbersWithUniqueDigits(check.n); got != check.want {
			t.Fatalf("countNumbersWithUniqueDigits(%d) = %d, want %d", check.n, got, check.want)
		}
	}
}
