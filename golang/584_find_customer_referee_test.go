package leetcode

import (
	"strings"
	"testing"
)

func TestFindCustomerRefereeSQL(t *testing.T) {
	query := strings.ToLower(findCustomerRefereeSQL)
	if !strings.Contains(query, "referee_id != 2") || !strings.Contains(query, "referee_id is null") {
		t.Fatalf("findCustomerRefereeSQL = %q, want customers whose referee is not 2", findCustomerRefereeSQL)
	}
}
