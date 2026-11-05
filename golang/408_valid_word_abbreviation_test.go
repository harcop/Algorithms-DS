package leetcode

import "testing"

func TestValidWordAbbreviationExamples(t *testing.T) {
	if !validWordAbbreviation("internationalization", "i12iz4n") {
		t.Fatal("validWordAbbreviation(i12iz4n) = false, want true")
	}
	if validWordAbbreviation("apple", "a2e") {
		t.Fatal("validWordAbbreviation(a2e) = true, want false")
	}
	if validWordAbbreviation("a", "01") {
		t.Fatal("validWordAbbreviation(01) = true, want false")
	}
}
