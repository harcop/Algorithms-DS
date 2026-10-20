package leetcode

import (
	"strings"
	"testing"
)

func TestEmployeesEarningMoreSQL(t *testing.T) {
	if !strings.Contains(employeesEarningMoreSQL, "Salary") {
		t.Fatalf("employeesEarningMoreSQL = %q, want a salary comparison", employeesEarningMoreSQL)
	}
}
