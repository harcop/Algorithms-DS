package leetcode

// LeetCode #308 - Range Sum Query 2D - Mutable
//
// LeetCode names this NumMatrix; that name already belongs to the immutable prefix matrix (#304).
// Keep the matrix and add the cells inside each requested rectangle.
type NumMatrixII struct {
	grid [][]int
}

func NewNumMatrixII(matrix [][]int) NumMatrixII {
	grid := make([][]int, len(matrix))
	for i := range matrix {
		grid[i] = append([]int(nil), matrix[i]...)
	}
	return NumMatrixII{grid: grid}
}

func (matrix *NumMatrixII) Update(row, col, val int) {
	matrix.grid[row][col] = val
}

func (matrix *NumMatrixII) SumRegion(row1, col1, row2, col2 int) int {
	sum := 0
	for i := row1; i <= row2; i++ {
		for j := col1; j <= col2; j++ {
			sum += matrix.grid[i][j]
		}
	}
	return sum
}
