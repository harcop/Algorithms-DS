package leetcode

import (
	"math"
	"testing"
)

func TestMovingAverageRolling(t *testing.T) {
	m := NewMovingAverage(3)
	checks := []struct {
		value int
		want  float64
	}{
		{1, 1},
		{10, 5.5},
		{3, 4.666666666666667},
		{5, 6},
	}
	for _, check := range checks {
		got := m.Next(check.value)
		if math.Abs(got-check.want) > 1e-6 {
			t.Fatalf("Next(%d) = %v, want %v", check.value, got, check.want)
		}
	}
}
