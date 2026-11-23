package leetcode

import "testing"

func TestPredictPartyVictoryExamples(t *testing.T) {
	if got := predictPartyVictory("RD"); got != "Radiant" {
		t.Fatalf("predictPartyVictory(RD) = %q, want Radiant", got)
	}
	if got := predictPartyVictory("RDD"); got != "Dire" {
		t.Fatalf("predictPartyVictory(RDD) = %q, want Dire", got)
	}
}
