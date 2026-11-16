package leetcode

// LeetCode #558 - Logical OR of Two Binary Grids Represented as Quad-Trees
//
// LeetCode names this intersect; that name already belongs to the two arrays (#350).
// A true leaf covers the other tree, a false leaf disappears, and four equal leaves collapse into one.
func intersectQuad(first, second *QuadNode) *QuadNode {
	if first == nil {
		return second
	}
	if second == nil {
		return first
	}
	if first.IsLeaf {
		if first.Val {
			return first
		}
		return second
	}
	if second.IsLeaf {
		if second.Val {
			return second
		}
		return first
	}
	topLeft := intersectQuad(first.TopLeft, second.TopLeft)
	topRight := intersectQuad(first.TopRight, second.TopRight)
	bottomLeft := intersectQuad(first.BottomLeft, second.BottomLeft)
	bottomRight := intersectQuad(first.BottomRight, second.BottomRight)
	if topLeft.IsLeaf && topRight.IsLeaf && bottomLeft.IsLeaf && bottomRight.IsLeaf &&
		topLeft.Val == topRight.Val && topRight.Val == bottomLeft.Val && bottomLeft.Val == bottomRight.Val {
		return &QuadNode{Val: topLeft.Val, IsLeaf: true}
	}
	return &QuadNode{
		IsLeaf:      false,
		TopLeft:     topLeft,
		TopRight:    topRight,
		BottomLeft:  bottomLeft,
		BottomRight: bottomRight,
	}
}
