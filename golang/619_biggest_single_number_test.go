package leetcode

import (
	"strings"
	"testing"
)

func TestBiggestSingleNumberSQL(t *testing.T) {
	query := strings.ToLower(biggestSingleNumberSQL)
	for _, piece := range []string{"max(num)", "group by num", "having count(*) = 1"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("biggestSingleNumberSQL = %q, want %q", biggestSingleNumberSQL, piece)
		}
	}
}
