package leetcode

import (
	"strings"
	"testing"
)

func TestCumulativeSalarySQL(t *testing.T) {
	query := strings.ToLower(cumulativeSalarySQL)
	if !strings.Contains(query, "max(month)") || !strings.Contains(query, "month - 2") {
		t.Fatalf("cumulativeSalarySQL = %q, want a three-month sum excluding the latest month", cumulativeSalarySQL)
	}
}
