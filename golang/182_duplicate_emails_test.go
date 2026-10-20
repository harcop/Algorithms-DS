package leetcode

import (
	"strings"
	"testing"
)

func TestDuplicateEmailsSQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(duplicateEmailsSQL), "group by") {
		t.Fatalf("duplicateEmailsSQL = %q, want GROUP BY", duplicateEmailsSQL)
	}
}
