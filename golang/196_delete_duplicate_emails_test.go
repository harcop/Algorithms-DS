package leetcode

import (
	"strings"
	"testing"
)

func TestDeleteDuplicateEmailsSQL(t *testing.T) {
	if !strings.Contains(strings.ToUpper(deleteDuplicateEmailsSQL), "DELETE") {
		t.Fatalf("deleteDuplicateEmailsSQL = %q, want DELETE", deleteDuplicateEmailsSQL)
	}
}
