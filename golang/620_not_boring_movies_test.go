package leetcode

import (
	"strings"
	"testing"
)

func TestNotBoringMoviesSQL(t *testing.T) {
	query := strings.ToLower(notBoringMoviesSQL)
	for _, piece := range []string{"id % 2 = 1", "description != 'boring'", "order by rating desc"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("notBoringMoviesSQL = %q, want %q", notBoringMoviesSQL, piece)
		}
	}
}
