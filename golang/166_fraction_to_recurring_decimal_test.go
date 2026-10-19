package leetcode

import "testing"

func TestFractionToDecimalExampleOne(t *testing.T) {
	got := fractionToDecimal(1, 2)
	if got != "0.5" {
		t.Fatalf("fractionToDecimal() = %q, want %q", got, "0.5")
	}
}

func TestFractionToDecimalExampleTwo(t *testing.T) {
	got := fractionToDecimal(2, 1)
	if got != "2" {
		t.Fatalf("fractionToDecimal() = %q, want %q", got, "2")
	}
}

func TestFractionToDecimalExampleThree(t *testing.T) {
	got := fractionToDecimal(4, 333)
	if got != "0.(012)" {
		t.Fatalf("fractionToDecimal() = %q, want %q", got, "0.(012)")
	}
}
