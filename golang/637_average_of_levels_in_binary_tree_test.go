package leetcode

import "testing"

func TestAverageOfLevelsExample(t *testing.T) {
	root := makeTree(3, makeTree(9, nil, nil), makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)))
	got := averageOfLevels(root)
	want := []float64{3, 14.5, 11}
	if len(got) != len(want) {
		t.Fatalf("averageOfLevels() = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("averageOfLevels() = %v, want %v", got, want)
		}
	}
}
