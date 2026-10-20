package leetcode

import (
	"strings"
	"testing"
)

func TestConsecutiveNumbersSQL(t *testing.T) {
	if strings.Count(consecutiveNumbersSQL, "JOIN") < 2 {
		t.Fatalf("consecutiveNumbersSQL = %q, want two JOINs", consecutiveNumbersSQL)
	}
}
