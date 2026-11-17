package leetcode

import (
	"strings"
	"testing"
)

func TestWinningCandidateSQL(t *testing.T) {
	query := strings.ToLower(winningCandidateSQL)
	if !strings.Contains(query, "candidateid") || !strings.Contains(query, "count(*) desc") {
		t.Fatalf("winningCandidateSQL = %q, want the candidate with the most votes", winningCandidateSQL)
	}
}
