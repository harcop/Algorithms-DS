package leetcode

// QuadNode is one cell of a quad tree. A leaf covers a uniform block of the grid.
type QuadNode struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *QuadNode
	TopRight    *QuadNode
	BottomLeft  *QuadNode
	BottomRight *QuadNode
}

// LeetCode #427 - Construct Quad Tree
//
// A region that is all one value becomes a leaf; otherwise it splits into four quadrants.
func constructQuadTree(grid [][]int) *QuadNode {
	var build func(r, c, size int) *QuadNode
	build = func(r, c, size int) *QuadNode {
		same := true
		for i := r; i < r+size && same; i++ {
			for j := c; j < c+size; j++ {
				if grid[i][j] != grid[r][c] {
					same = false
					break
				}
			}
		}
		if same {
			return &QuadNode{Val: grid[r][c] == 1, IsLeaf: true}
		}
		half := size / 2
		return &QuadNode{
			IsLeaf:      false,
			TopLeft:     build(r, c, half),
			TopRight:    build(r, c+half, half),
			BottomLeft:  build(r+half, c, half),
			BottomRight: build(r+half, c+half, half),
		}
	}
	if len(grid) == 0 {
		return nil
	}
	return build(0, 0, len(grid))
}
