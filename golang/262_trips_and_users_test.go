package leetcode

import (
	"strings"
	"testing"
)

func TestTripsAndUsersSQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(tripsAndUsersSQL), "trips") {
		t.Fatalf("tripsAndUsersSQL = %q, want a trips query", tripsAndUsersSQL)
	}
}
