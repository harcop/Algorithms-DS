package leetcode

import (
	"strings"
	"testing"
)

func TestStudentsByGeographySQL(t *testing.T) {
	query := strings.ToLower(studentsByGeographySQL)
	for _, piece := range []string{"'america'", "'asia'", "'europe'", "row_number()", "order by name", "group by rk"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("studentsByGeographySQL = %q, want %q", studentsByGeographySQL, piece)
		}
	}
}
