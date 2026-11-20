package leetcode

import (
	"strings"
	"testing"
)

func TestMostFriendsSQL(t *testing.T) {
	query := strings.ToLower(mostFriendsSQL)
	for _, piece := range []string{"requester_id", "accepter_id", "union all", "count(*)", "limit 1"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("mostFriendsSQL = %q, want %q", mostFriendsSQL, piece)
		}
	}
}
