package leetcode

import (
	"math"
	"testing"
)

func TestMyPowExampleOne(t *testing.T) {
	got := myPow(2.0, 10)
	if math.Abs(got-1024.0) > 1e-9 {
		t.Fatalf("myPow() = %v, want 1024", got)
	}
}

func TestMyPowExampleTwo(t *testing.T) {
	got := myPow(2.1, 3)
	if math.Abs(got-9.261) > 1e-6 {
		t.Fatalf("myPow() = %v, want 9.261", got)
	}
}

func TestMyPowExampleThree(t *testing.T) {
	got := myPow(2.0, -2)
	if math.Abs(got-0.25) > 1e-9 {
		t.Fatalf("myPow() = %v, want 0.25", got)
	}
}
