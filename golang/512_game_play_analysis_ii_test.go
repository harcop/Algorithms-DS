package leetcode

import (
	"strings"
	"testing"
)

func TestGamePlayAnalysisIISQL(t *testing.T) {
	query := strings.ToLower(gamePlayAnalysisIISQL)
	if !strings.Contains(query, "device_id") || !strings.Contains(query, "min(event_date)") {
		t.Fatalf("gamePlayAnalysisIISQL = %q, want the device from the first login", gamePlayAnalysisIISQL)
	}
}
