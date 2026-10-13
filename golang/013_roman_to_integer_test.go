package leetcode

import "testing"

func TestRomanToIntExampleOne(t *testing.T) {
	if got := romanToInt("III"); got != 3 {
		t.Fatalf("romanToInt(%q) = %d, want 3", "III", got)
	}
}

func TestRomanToIntExampleTwo(t *testing.T) {
	if got := romanToInt("LVIII"); got != 58 {
		t.Fatalf("romanToInt(%q) = %d, want 58", "LVIII", got)
	}
}

func TestRomanToIntExampleThree(t *testing.T) {
	if got := romanToInt("MCMXCIV"); got != 1994 {
		t.Fatalf("romanToInt(%q) = %d, want 1994", "MCMXCIV", got)
	}
}
