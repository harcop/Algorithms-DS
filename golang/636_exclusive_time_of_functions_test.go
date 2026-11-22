package leetcode

import "testing"

func TestExclusiveTimeExamples(t *testing.T) {
	got := exclusiveTime(2, []string{"0:start:0", "1:start:2", "1:end:5", "0:end:6"})
	if !sameInts(got, []int{3, 4}) {
		t.Fatalf("exclusiveTime() = %v, want [3 4]", got)
	}
	got = exclusiveTime(1, []string{"0:start:0", "0:start:2", "0:end:5", "0:start:6", "0:end:6", "0:end:7"})
	if !sameInts(got, []int{8}) {
		t.Fatalf("exclusiveTime() = %v, want [8]", got)
	}
}
