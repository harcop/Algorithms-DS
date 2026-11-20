package leetcode

import (
	"strings"
	"testing"
)

func TestShortestDistanceLineSQL(t *testing.T) {
	query := strings.ToLower(shortestDistanceLineSQL)
	for _, piece := range []string{"min(", "abs(", "from point", "as shortest"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("shortestDistanceLineSQL = %q, want %q", shortestDistanceLineSQL, piece)
		}
	}
}
