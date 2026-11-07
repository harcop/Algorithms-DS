package leetcode

import "testing"

func TestLevelOrderNaryExample(t *testing.T) {
	got := levelOrderNary(sampleNary())
	want := [][]int{{1}, {3, 2, 4}, {5, 6}}
	if len(got) != len(want) {
		t.Fatalf("levelOrderNary() = %v, want %v", got, want)
	}
	for i := range want {
		if !sameInts(got[i], want[i]) {
			t.Fatalf("levelOrderNary() = %v, want %v", got, want)
		}
	}
	if levelOrderNary(nil) != nil {
		t.Fatal("levelOrderNary(nil) != nil")
	}
}
