package leetcode

import (
	"strings"
	"testing"
)

func TestClassesWithAtLeast5StudentsSQL(t *testing.T) {
	query := strings.ToLower(classesWithAtLeast5StudentsSQL)
	if !strings.Contains(query, "group by class") || !strings.Contains(query, "count(*) >= 5") {
		t.Fatalf("classesWithAtLeast5StudentsSQL = %q, want classes of five or more", classesWithAtLeast5StudentsSQL)
	}
}
