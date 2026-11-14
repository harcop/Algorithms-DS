package leetcode

import (
	"strings"
	"testing"
)

func TestGamePlayAnalysisIIISQL(t *testing.T) {
	query := strings.ToLower(gamePlayAnalysisIIISQL)
	if !strings.Contains(query, "sum(games_played)") || !strings.Contains(query, "partition by player_id") {
		t.Fatalf("gamePlayAnalysisIIISQL = %q, want the running games total for each player", gamePlayAnalysisIIISQL)
	}
}
