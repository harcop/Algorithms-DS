package leetcode

import (
	"strings"
	"testing"
)

func TestSwapSalarySQL(t *testing.T) {
	query := strings.ToLower(swapSalarySQL)
	for _, piece := range []string{"update salary", "set sex", "when 'm' then 'f'", "else 'm'"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("swapSalarySQL = %q, want %q", swapSalarySQL, piece)
		}
	}
}
