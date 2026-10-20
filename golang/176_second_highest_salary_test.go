package leetcode

import (
	"strings"
	"testing"
)

func TestSecondHighestSalarySQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(secondHighestSalarySQL), "salary") {
		t.Fatalf("secondHighestSalarySQL = %q, want a salary query", secondHighestSalarySQL)
	}
}
