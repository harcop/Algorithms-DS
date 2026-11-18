package leetcode

import (
	"strings"
	"testing"
)

func TestBigCountriesSQL(t *testing.T) {
	query := strings.ToLower(bigCountriesSQL)
	if !strings.Contains(query, "area >= 3000000") || !strings.Contains(query, "population >= 25000000") {
		t.Fatalf("bigCountriesSQL = %q, want a large area or a large population", bigCountriesSQL)
	}
}
