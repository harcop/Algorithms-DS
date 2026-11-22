package leetcode

import (
	"strconv"
	"strings"
)

// LeetCode #631 - Design Excel Sum Formula
//
// A sum cell remembers its ranges and adds the current value of each cell when it is read.
type excelSpan struct {
	r1, c1, r2, c2 int
}

type Excel struct {
	vals     [][]int
	formulas [][][]excelSpan
}

func NewExcel(height int, width byte) *Excel {
	cols := int(width-'A') + 1
	sheet := &Excel{
		vals:     make([][]int, height),
		formulas: make([][][]excelSpan, height),
	}
	for row := 0; row < height; row++ {
		sheet.vals[row] = make([]int, cols)
		sheet.formulas[row] = make([][]excelSpan, cols)
	}
	return sheet
}

func (e *Excel) Set(row int, column byte, val int) {
	r, c := row-1, int(column-'A')
	e.vals[r][c] = val
	e.formulas[r][c] = nil
}

func (e *Excel) Get(row int, column byte) int {
	r, c := row-1, int(column-'A')
	if e.formulas[r][c] == nil {
		return e.vals[r][c]
	}
	total := 0
	for _, span := range e.formulas[r][c] {
		for i := span.r1; i <= span.r2; i++ {
			for j := span.c1; j <= span.c2; j++ {
				total += e.Get(i+1, byte('A'+j))
			}
		}
	}
	return total
}

func (e *Excel) Sum(row int, column byte, numbers []string) int {
	r, c := row-1, int(column-'A')
	spans := make([]excelSpan, 0, len(numbers))
	for _, token := range numbers {
		spans = append(spans, parseExcelSpan(token))
	}
	e.formulas[r][c] = spans
	return e.Get(row, column)
}

func parseExcelSpan(token string) excelSpan {
	parts := strings.Split(token, ":")
	r1, c1 := parseExcelCell(parts[0])
	r2, c2 := r1, c1
	if len(parts) == 2 {
		r2, c2 = parseExcelCell(parts[1])
	}
	if r1 > r2 {
		r1, r2 = r2, r1
	}
	if c1 > c2 {
		c1, c2 = c2, c1
	}
	return excelSpan{r1: r1, c1: c1, r2: r2, c2: c2}
}

func parseExcelCell(cell string) (int, int) {
	row, _ := strconv.Atoi(cell[1:])
	return row - 1, int(cell[0] - 'A')
}
