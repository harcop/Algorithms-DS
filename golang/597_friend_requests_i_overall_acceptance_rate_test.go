package leetcode

import (
	"strings"
	"testing"
)

func TestFriendAcceptanceRateSQL(t *testing.T) {
	query := strings.ToLower(friendAcceptanceRateSQL)
	if !strings.Contains(query, "accept_rate") || !strings.Contains(query, "requestaccepted") {
		t.Fatalf("friendAcceptanceRateSQL = %q, want accepted pairs over sent pairs", friendAcceptanceRateSQL)
	}
}
