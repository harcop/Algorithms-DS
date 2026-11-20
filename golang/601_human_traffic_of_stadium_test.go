package leetcode

import (
	"strings"
	"testing"
)

func TestHumanTrafficSQL(t *testing.T) {
	query := strings.ToLower(humanTrafficSQL)
	for _, piece := range []string{"people >= 100", "row_number()", "count(*) >= 3", "order by visit_date"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("humanTrafficSQL = %q, want %q", humanTrafficSQL, piece)
		}
	}
}
