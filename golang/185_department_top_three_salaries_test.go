package leetcode

import (
	"strings"
	"testing"
)

func TestDepartmentTopThreeSalariesSQL(t *testing.T) {
	if !strings.Contains(departmentTopThreeSalariesSQL, "<= 3") {
		t.Fatalf("departmentTopThreeSalariesSQL = %q, want the top-three cutoff", departmentTopThreeSalariesSQL)
	}
}
