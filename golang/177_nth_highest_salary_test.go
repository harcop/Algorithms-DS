package leetcode

import (
	"strings"
	"testing"
)

func TestNthHighestSalarySQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(nthHighestSalarySQL), "function") {
		t.Fatalf("nthHighestSalarySQL = %q, want a function definition", nthHighestSalarySQL)
	}
}
