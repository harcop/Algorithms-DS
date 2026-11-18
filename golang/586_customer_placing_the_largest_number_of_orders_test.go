package leetcode

import (
	"strings"
	"testing"
)

func TestLargestNumberOfOrdersSQL(t *testing.T) {
	query := strings.ToLower(largestNumberOfOrdersSQL)
	if !strings.Contains(query, "customer_number") || !strings.Contains(query, "limit 1") {
		t.Fatalf("largestNumberOfOrdersSQL = %q, want the customer with the most orders", largestNumberOfOrdersSQL)
	}
}
