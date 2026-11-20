package leetcode

import (
	"strings"
	"testing"
)

func TestConsecutiveSeatsSQL(t *testing.T) {
	query := strings.ToLower(consecutiveSeatsSQL)
	for _, piece := range []string{"seat_id", "free = 1", "abs(", "order by a.seat_id"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("consecutiveSeatsSQL = %q, want %q", consecutiveSeatsSQL, piece)
		}
	}
}
