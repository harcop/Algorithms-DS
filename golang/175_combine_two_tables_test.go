package leetcode

import (
	"strings"
	"testing"
)

func TestCombineTwoTablesSQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(combineTwoTablesSQL), "left join") {
		t.Fatalf("combineTwoTablesSQL = %q, want a LEFT JOIN", combineTwoTablesSQL)
	}
}
