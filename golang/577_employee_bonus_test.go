package leetcode

import (
	"strings"
	"testing"
)

func TestEmployeeBonusSQL(t *testing.T) {
	query := strings.ToLower(employeeBonusSQL)
	if !strings.Contains(query, "left join bonus") || !strings.Contains(query, "bonus < 1000") {
		t.Fatalf("employeeBonusSQL = %q, want bonuses below 1000, including none", employeeBonusSQL)
	}
}
