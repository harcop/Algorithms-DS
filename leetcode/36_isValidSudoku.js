// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
// Note:

// A Sudoku board (partially filled) could be valid but is not necessarily solvable.
// Only the filled cells need to be validated according to the mentioned rules.

const isValidSudoku = function(board) {
    const boardLength = board.length;
    const groupCells = []
    for(let i = 0; i < boardLength; i++) {
        const currentRow = board[i];
        for(let ii = 0; ii < currentRow.length; ii++) {
            // ii = board[0][0] = [i][ii]
            // row check
            const currentValue = currentRow[ii];
            if(Number(currentValue)) {
                for(let j = ii + 1; j < boardLength; j++) {
                    // j = board[0][1]
                    if(currentRow[j] === currentValue) {
                        return false
                    }
                }

                // column check;
                const currentColumn = i;
                for(let cc = currentColumn + 1; cc < boardLength; cc++) {
                    if(board[cc][ii] === currentValue) {
                        return false;
                    }
                }

              }
              // cellcheck
              let startCells = [0, 3, 6];
              if(startCells.includes(ii) && startCells.includes(i)) {
                  // group cells
                  const cell = []
                  for(let q = i; q < i + 3; q++) {
                      for(let w = ii; w < ii + 3; w++) {
                          const cellValue = board[q][w]
                          if(Number(cellValue)) {
                              cell.push(cellValue);
                          }
                      }
                  }
                  groupCells.push(cell);
              }

        }
    }
    // compare grid value
    for(let groupCell of groupCells) {
        for(let cc = 0; cc < groupCell.length - 1; cc++) {
            const cellValue = groupCell[cc];
            for(let dc = cc + 1; dc < groupCell.length; dc++) {
                if(groupCell[dc] === cellValue) {
                    return false;
                }
            }
        }
    }
    return true
};
const board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
console.log(isValidSudoku(board))

console.log(isValidSudoku([["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]))
