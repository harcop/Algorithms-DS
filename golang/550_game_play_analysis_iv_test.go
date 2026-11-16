package leetcode

import (
	"strings"
	"testing"
)

func TestGamePlayAnalysisIVSQL(t *testing.T) {
	query := strings.ToLower(gamePlayAnalysisIVSQL)
	if !strings.Contains(query, "round(") || !strings.Contains(query, "date_add") || !strings.Contains(query, "min(event_date)") {
		t.Fatalf("gamePlayAnalysisIVSQL = %q, want the next-day login fraction", gamePlayAnalysisIVSQL)
	}
}
