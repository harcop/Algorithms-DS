package leetcode

import "testing"

func TestNearestPalindromicExamples(t *testing.T) {
	cases := []struct {
		n, want string
	}{
		{"123", "121"},
		{"1", "0"},
		{"10", "9"},
		{"99", "101"},
		{"121", "111"},
	}
	for _, tc := range cases {
		if got := nearestPalindromic(tc.n); got != tc.want {
			t.Fatalf("nearestPalindromic(%s) = %s, want %s", tc.n, got, tc.want)
		}
	}
}
