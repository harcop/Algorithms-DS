package leetcode

import (
	"strings"
	"testing"
)

func TestGamePlayAnalysisISQL(t *testing.T) {
	query := strings.ToLower(gamePlayAnalysisISQL)
	if !strings.Contains(query, "min(event_date)") || !strings.Contains(query, "group by player_id") {
		t.Fatalf("gamePlayAnalysisISQL = %q, want the first login grouped by player", gamePlayAnalysisISQL)
	}
}
