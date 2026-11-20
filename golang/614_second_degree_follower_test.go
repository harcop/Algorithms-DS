package leetcode

import (
	"strings"
	"testing"
)

func TestSecondDegreeFollowerSQL(t *testing.T) {
	query := strings.ToLower(secondDegreeFollowerSQL)
	for _, piece := range []string{"f1.follower = f2.followee", "count(distinct", "order by f1.follower"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("secondDegreeFollowerSQL = %q, want %q", secondDegreeFollowerSQL, piece)
		}
	}
}
