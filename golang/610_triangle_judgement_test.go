package leetcode

import (
	"strings"
	"testing"
)

func TestTriangleJudgementSQL(t *testing.T) {
	query := strings.ToLower(triangleJudgementSQL)
	for _, piece := range []string{"x + y > z", "x + z > y", "y + z > x", "'yes'", "'no'"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("triangleJudgementSQL = %q, want %q", triangleJudgementSQL, piece)
		}
	}
}
