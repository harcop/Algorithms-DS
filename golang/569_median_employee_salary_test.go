package leetcode

import (
	"strings"
	"testing"
)

func TestMedianEmployeeSalarySQL(t *testing.T) {
	query := strings.ToLower(medianEmployeeSalarySQL)
	if !strings.Contains(query, "row_number()") || !strings.Contains(query, "partition by company") {
		t.Fatalf("medianEmployeeSalarySQL = %q, want a rank within each company", medianEmployeeSalarySQL)
	}
}
