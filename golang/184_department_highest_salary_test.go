package leetcode

import (
	"strings"
	"testing"
)

func TestDepartmentHighestSalarySQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(departmentHighestSalarySQL), "max(salary)") {
		t.Fatalf("departmentHighestSalarySQL = %q, want MAX(Salary)", departmentHighestSalarySQL)
	}
}
