package leetcode

import (
	"strings"
	"testing"
)

func TestTreeNodeTypeSQL(t *testing.T) {
	query := strings.ToLower(treeNodeTypeSQL)
	for _, piece := range []string{"p_id is null", "'root'", "'inner'", "'leaf'"} {
		if !strings.Contains(query, piece) {
			t.Fatalf("treeNodeTypeSQL = %q, want %q", treeNodeTypeSQL, piece)
		}
	}
}
