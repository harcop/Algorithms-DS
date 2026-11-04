package leetcode

import (
	"math"
	"testing"
)

func TestCalcEquationExample(t *testing.T) {
	equations := [][]string{{"a", "b"}, {"b", "c"}}
	values := []float64{2, 3}
	queries := [][]string{{"a", "c"}, {"b", "a"}, {"a", "e"}, {"a", "a"}, {"x", "x"}}
	got := calcEquation(equations, values, queries)
	want := []float64{6, 0.5, -1, 1, -1}
	if len(got) != len(want) {
		t.Fatalf("calcEquation() = %v, want %v", got, want)
	}
	for i := range want {
		if math.Abs(got[i]-want[i]) > 1e-9 {
			t.Fatalf("calcEquation() = %v, want %v", got, want)
		}
	}
}
