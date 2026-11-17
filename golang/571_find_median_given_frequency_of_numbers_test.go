package leetcode

import (
	"strings"
	"testing"
)

func TestMedianFrequencySQL(t *testing.T) {
	query := strings.ToLower(medianFrequencySQL)
	if !strings.Contains(query, "avg(num)") || !strings.Contains(query, "frequency") {
		t.Fatalf("medianFrequencySQL = %q, want the median weighted by frequency", medianFrequencySQL)
	}
}
