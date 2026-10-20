package leetcode

import (
	"strings"
	"testing"
)

func TestRankScoresSQL(t *testing.T) {
	if !strings.Contains(rankScoresSQL, "Rank") {
		t.Fatalf("rankScoresSQL = %q, want a Rank column", rankScoresSQL)
	}
}
