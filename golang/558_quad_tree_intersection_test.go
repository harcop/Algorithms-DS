package leetcode

import "testing"

func TestIntersectQuadExamples(t *testing.T) {
	got := intersectQuad(&QuadNode{Val: true, IsLeaf: true}, &QuadNode{Val: false, IsLeaf: true})
	if got == nil || !got.IsLeaf || !got.Val {
		t.Fatalf("intersectQuad(true, false) = %+v, want a true leaf", got)
	}
	first := &QuadNode{
		TopLeft:     &QuadNode{Val: true, IsLeaf: true},
		TopRight:    &QuadNode{Val: true, IsLeaf: true},
		BottomLeft:  &QuadNode{Val: false, IsLeaf: true},
		BottomRight: &QuadNode{Val: false, IsLeaf: true},
	}
	second := &QuadNode{
		TopLeft:     &QuadNode{Val: true, IsLeaf: true},
		TopRight:    &QuadNode{Val: true, IsLeaf: true},
		BottomLeft:  &QuadNode{Val: true, IsLeaf: true},
		BottomRight: &QuadNode{Val: false, IsLeaf: true},
	}
	merged := intersectQuad(first, second)
	if merged == nil || merged.IsLeaf ||
		!merged.TopLeft.Val || !merged.TopRight.Val || !merged.BottomLeft.Val || merged.BottomRight.Val {
		t.Fatalf("intersectQuad = %+v, want the four quadrants true, true, true, false", merged)
	}
}
