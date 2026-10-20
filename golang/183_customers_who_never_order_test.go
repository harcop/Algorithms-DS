package leetcode

import (
	"strings"
	"testing"
)

func TestCustomersWhoNeverOrderSQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(customersWhoNeverOrderSQL), "left join") {
		t.Fatalf("customersWhoNeverOrderSQL = %q, want a LEFT JOIN", customersWhoNeverOrderSQL)
	}
}
