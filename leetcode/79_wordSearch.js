/**
 * @param {character[][]} board
 * @param {string} word
 * @return {boolean}
 */
var exist = function(board, word) {

  function enterSearch(startI, startJ, wordIndex, positionArray, foundLetters, breaker) {
    for(let i = startI; i < board.length; i++) {
      for(let j = startJ; j < board[0].length; j++) {
        let ele = board[i][j];
        let pos = `${i}${j}`;
        if(ele === word[wordIndex] && !positionArray.includes(pos)) {
          foundLetters += ele
          let found = false
          if(foundLetters === word) {
            return true
          }
          // go forward
          if(j + 1 < board[0].length && board[i][j + 1] === word[wordIndex + 1]) {
            found = enterSearch(i, j+1, wordIndex + 1, [...positionArray, pos], foundLetters, true)
            if(found) return true
          }
          // go down
          if(i + 1 < board.length && board[i+1][j] === word[wordIndex + 1]) {
            found = enterSearch(i+1, j, wordIndex + 1, [...positionArray, pos], foundLetters, true)
            if(found) return true
          }
          // go up
          if(i > 0 && board[i-1][j] === word[wordIndex + 1]) {
            found = enterSearch(i-1, j, wordIndex + 1, [...positionArray, pos], foundLetters, true)
            if(found) return true
          }
          // go back
          if(j > 0 && board[i][j-1] === word[wordIndex + 1]) {
            found = enterSearch(i, j-1, wordIndex + 1, [...positionArray, pos], foundLetters, true)
            if(found) return true
          }
          foundLetters = foundLetters.slice(0, -1)
        }
        if(breaker) {
          return false
        }
      }
    }
  }

  return enterSearch(0, 0, 0, [], '', false) || false
};
console.log(exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED"))
console.log(exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "SEE"))
console.log(exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCB"))
