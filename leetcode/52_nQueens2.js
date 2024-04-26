/**
 * @param {number} n
 * @return {string[][]}
 */
var solveNQueens = function(n) {
    // create n x n grid
    const board = [];
    for(let i = 0; i < n; i++) {
      board.push(new Array(n).fill('.'))
    }

    const validBoard = Array.from(board.map(row => [...row]));

    const result = [];
    let count = 0;
    placeQueen(board, 0, 0, validBoard, result, count)

    return result.length;
};

function placeQueen(board, startY, startX, validBoard, result, count) {
    const n = board.length;
    for(let y = startY; y < n; y++) {
      for(let x = startX; x < n; x++) {
        // place queen
        if(isValidPosition(validBoard, y, x)) {
          const validBoardInner = Array.from(validBoard.map(row => [...row]));
          validBoardInner[y][x] = 'Q';
          let nnCount = count + 1;
          placeQueen(board, y, 0, validBoardInner, result, nnCount);
          if(nnCount === n) {
            result.push(validBoardInner.map(ele => ele.join('')))
          }
        }
      }
    }
}

function isValidPosition(board, y, x) {

  if(x >= board.length || y >= board.length || x < 0 || y < 0) {
    return false
  } 

  // check for x-axis
  for(let i = 0; i < board.length; i++) {
    if(board[y][i] === 'Q') {
      return false
    }
  }

  // check for y-axis
  for(let i = 0; i < board.length; i++) {
    if(board[i][x] === 'Q') {
      return false
    }
  }

  // check for diagonal
  // N-W
  let nwx = x;
  let nwy = y;
  while(board[nwy][nwx]) {
    if(board[nwy][nwx] === 'Q') {
      return false
    }
    nwx -= 1;
    nwy -= 1;
    if(nwx < 0 || nwy < 0) {
      break;
    }
  }

  // N-E
  let nex = x;
  let ney = y;
  while(board[ney][nex]) {
    if(board[ney][nex] === 'Q') {
      return false
    }
    nex += 1;
    ney -= 1;
    if(nex >= board.length || ney < 0) {
      break;
    }
  }

  // S-W
  let swx = x;
  let swy = y;
  while(board[swy][swx]) {
    if(board[swy][swx] === 'Q') {
      return false
    }
    swx -= 1;
    swy += 1;
    if(swx < 0 || swy >= board.length) {
      break;
    }
  }

  // S-E
  let sex = x;
  let sey = y;
  while(board[sey][sex]) {
    if(board[sey][sex] === 'Q') {
      return false
    }
    sex += 1;
    sey += 1;
    if(sex >= board.length || sey >= board.length) {
      break;
    }
  }

  return true
}

console.log(solveNQueens(1))
console.log(solveNQueens(2))
console.log(solveNQueens(4))
