package leetcode

import (
	"strings"
	"testing"
)

func TestStudentCountByDepartmentSQL(t *testing.T) {
	query := strings.ToLower(studentCountByDepartmentSQL)
	if !strings.Contains(query, "left join student") || !strings.Contains(query, "count(s.student_id)") {
		t.Fatalf("studentCountByDepartmentSQL = %q, want a student count for every department", studentCountByDepartmentSQL)
	}
}
