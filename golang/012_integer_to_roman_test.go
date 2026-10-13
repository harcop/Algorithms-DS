package leetcode

import "testing"

func TestIntToRomanExampleOne(t *testing.T) {
	if got := intToRoman(3); got != "III" {
		t.Fatalf("intToRoman(3) = %q, want %q", got, "III")
	}
}

func TestIntToRomanExampleTwo(t *testing.T) {
	if got := intToRoman(58); got != "LVIII" {
		t.Fatalf("intToRoman(58) = %q, want %q", got, "LVIII")
	}
}

func TestIntToRomanExampleThree(t *testing.T) {
	if got := intToRoman(1994); got != "MCMXCIV" {
		t.Fatalf("intToRoman(1994) = %q, want %q", got, "MCMXCIV")
	}
}
