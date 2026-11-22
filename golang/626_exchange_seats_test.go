package leetcode

import (
	"strings"
	"testing"
)

func TestExchangeSeatsSQL(t *testing.T) {
	query := strings.ToLower(exchangeSeatsSQL)
	for _, piece := range []string{"id % 2 = 1", "id - 1", "order by id", "student"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("exchangeSeatsSQL = %q, want %q", exchangeSeatsSQL, piece)
		}
	}
}
