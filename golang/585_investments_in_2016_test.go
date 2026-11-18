package leetcode

import (
	"strings"
	"testing"
)

func TestInvestmentsIn2016SQL(t *testing.T) {
	query := strings.ToLower(investmentsIn2016SQL)
	if !strings.Contains(query, "tiv_2016") || !strings.Contains(query, "having count(*) > 1") {
		t.Fatalf("investmentsIn2016SQL = %q, want shared 2015 values in a unique city", investmentsIn2016SQL)
	}
}
