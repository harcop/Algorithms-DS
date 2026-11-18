package leetcode

import "testing"

func TestFractionAdditionExamples(t *testing.T) {
	if got := fractionAddition("-1/2+1/2"); got != "0/1" {
		t.Fatalf("fractionAddition = %s, want 0/1", got)
	}
	if got := fractionAddition("-1/2+1/2+1/3"); got != "1/3" {
		t.Fatalf("fractionAddition = %s, want 1/3", got)
	}
	if got := fractionAddition("1/3-1/2"); got != "-1/6" {
		t.Fatalf("fractionAddition = %s, want -1/6", got)
	}
}
