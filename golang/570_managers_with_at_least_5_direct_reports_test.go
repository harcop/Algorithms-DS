package leetcode

import (
	"strings"
	"testing"
)

func TestManagersWithAtLeast5ReportsSQL(t *testing.T) {
	query := strings.ToLower(managersWithAtLeast5ReportsSQL)
	if !strings.Contains(query, "managerid") || !strings.Contains(query, "count(*) >= 5") {
		t.Fatalf("managersWithAtLeast5ReportsSQL = %q, want managers with five direct reports", managersWithAtLeast5ReportsSQL)
	}
}
