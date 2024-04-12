// Function to check if a number is valid in the current row, column, and 3x3 box
function isValid(board, row, col, num) {
  // Check row
  for (let i = 0; i < 9; i++) {
    if (board[row][i] === num) {
      return false;
    }
  }

  // Check column
  for (let i = 0; i < 9; i++) {
    if (board[i][col] === num) {
      return false;
    }
  }

  // Check 3x3 box
  const boxRow = Math.floor(row / 3);
  const boxCol = Math.floor(col / 3);

  for (let i = boxRow * 3; i < boxRow * 3 + 3; i++) {
    for (let j = boxCol * 3; j < boxCol * 3 + 3; j++) {
      if (board[i][j] === num) {
        return false;
      }
    }
  }

  return true;
}

// Function to solve the Sudoku puzzle
function solveSudoku(board) {
  for (let row = 0; row < 9; row++) {
    for (let col = 0; col < 9; col++) {
      if (board[row][col] === '.') {
        for (let num = 1; num <= 9; num++) {
          if (isValid(board, row, col, String(num))) {
            board[row][col] = String(num);

            if (solveSudoku(board)) {
              return true;
            }

            board[row][col] = '.';
          }
        }
        return false;
      }
    }
  }
  return true;
}

// Example usage
const board = [
  ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
  ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
  ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
  ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
  ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
  ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
  ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
  ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
  ['.', '.', '.', '.', '8', '.', '.', '7', '9']
];

solveSudoku(board);

console.log(board);
