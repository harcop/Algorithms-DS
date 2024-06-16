// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.

// Note: You can only move either down or right at any point in time.

/**
 * @param {number[][]} grid
 * @return {number}
 */
var minPathSum = function(grid) {
    let m = grid.length;
    let n = grid[0].length

    let arr = grid

    for(let y = m - 1; y >= 0; y--) {
      for(let x = n - 1; x >= 0; x--) {
        let right = 0;
        let down = 0;
        let min = Infinity;

        if(x + 1 < n) {
          right = arr[y][x + 1]
          min = Math.min(min, right);
        }
        // down cell
        if(y + 1 < m) {
          down = arr[y + 1][x]
          min = Math.min(min, down);
        }

        if(min !== Infinity) {
          arr[y][x] += min
        }
      }
    }
    return arr[0][0]
};


console.log(minPathSum([[1,3,1],[1,5,1],[4,2,1]]))
console.log(minPathSum([[1,2,3],[4,5,6]]))
