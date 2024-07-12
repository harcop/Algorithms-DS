/**
 * @param {character[][]} board
 * @param {string} word
 * @return {boolean}
 */
var exist = function(board, word) {
  let index = 0;

  for(let i = 0; i < board.length; i++) {
    for(let j = 0; j < board[0].length; j++) {
      let ele = board[i][j];
      if(ele === word[index]) {
        // go forward
        enterSearch()
        // go down
        // go up
        // go back
      }
    }
  }
  function enterSearch(index, positionArray) {
  }
};
