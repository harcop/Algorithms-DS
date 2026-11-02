package leetcode

import "testing"

func TestCanMeasureWaterExamples(t *testing.T) {
	if !canMeasureWater(3, 5, 4) {
		t.Fatal("canMeasureWater(3, 5, 4) = false, want true")
	}
	if canMeasureWater(2, 6, 5) {
		t.Fatal("canMeasureWater(2, 6, 5) = true, want false")
	}
	if !canMeasureWater(1, 2, 3) {
		t.Fatal("canMeasureWater(1, 2, 3) = false, want true")
	}
}
