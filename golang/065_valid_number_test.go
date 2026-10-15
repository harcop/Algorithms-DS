package leetcode

import "testing"

func TestIsNumberValidCases(t *testing.T) {
	for _, s := range []string{
		"2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1",
		"53.5e93", "-123.456e789",
	} {
		if !isNumber(s) {
			t.Fatalf("isNumber(%q) = false, want true", s)
		}
	}
}

func TestIsNumberInvalidCases(t *testing.T) {
	for _, s := range []string{"abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"} {
		if isNumber(s) {
			t.Fatalf("isNumber(%q) = true, want false", s)
		}
	}
}
