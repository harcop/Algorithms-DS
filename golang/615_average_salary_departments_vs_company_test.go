package leetcode

import (
	"strings"
	"testing"
)

func TestAverageSalaryComparisonSQL(t *testing.T) {
	query := strings.ToLower(averageSalaryComparisonSQL)
	for _, piece := range []string{"date_format", "department_avg", "company_avg", "'higher'", "'lower'", "'same'"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("averageSalaryComparisonSQL = %q, want %q", averageSalaryComparisonSQL, piece)
		}
	}
}
