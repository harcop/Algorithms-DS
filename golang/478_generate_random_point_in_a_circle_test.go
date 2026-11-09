package leetcode

import "testing"

func TestCirclePointExamples(t *testing.T) {
	circle := NewCirclePoint(1, 0, 0)
	for i := 0; i < 50; i++ {
		point := circle.RandPoint()
		if !insideCircle(point, 1, 0, 0) {
			t.Fatalf("RandPoint() = %v, outside the unit circle", point)
		}
	}
	circle = NewCirclePoint(10, 5, -7.5)
	for i := 0; i < 50; i++ {
		point := circle.RandPoint()
		if !insideCircle(point, 10, 5, -7.5) {
			t.Fatalf("RandPoint() = %v, outside the offset circle", point)
		}
	}
}

func insideCircle(point []float64, radius, x, y float64) bool {
	dx := point[0] - x
	dy := point[1] - y
	return dx*dx+dy*dy <= radius*radius+1e-9
}
