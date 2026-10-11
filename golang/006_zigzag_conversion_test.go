package leetcode

import "testing"

func TestConvertExampleOne(t *testing.T) {
	got := convert("PAYPALISHIRING", 3)
	if got != "PAHNAPLSIIGYIR" {
		t.Fatalf("convert() = %q, want %q", got, "PAHNAPLSIIGYIR")
	}
}

func TestConvertExampleTwo(t *testing.T) {
	got := convert("PAYPALISHIRING", 4)
	if got != "PINALSIGYAHRPI" {
		t.Fatalf("convert() = %q, want %q", got, "PINALSIGYAHRPI")
	}
}

func TestConvertExampleThree(t *testing.T) {
	got := convert("A", 1)
	if got != "A" {
		t.Fatalf("convert() = %q, want %q", got, "A")
	}
}
