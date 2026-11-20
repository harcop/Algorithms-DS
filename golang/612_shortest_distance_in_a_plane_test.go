package leetcode

import (
	"strings"
	"testing"
)

func TestShortestDistancePlaneSQL(t *testing.T) {
	query := strings.ToLower(shortestDistancePlaneSQL)
	for _, piece := range []string{"round(", "sqrt(", "min(", "point2d", "as shortest"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("shortestDistancePlaneSQL = %q, want %q", shortestDistancePlaneSQL, piece)
		}
	}
}
