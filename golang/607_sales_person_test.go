package leetcode

import (
	"strings"
	"testing"
)

func TestSalesPersonSQL(t *testing.T) {
	query := strings.ToLower(salesPersonSQL)
	for _, piece := range []string{"salesperson", "not in", "company", "'red'"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("salesPersonSQL = %q, want %q", salesPersonSQL, piece)
		}
	}
}
