/**
 * @param {number[][]} obstacleGrid
 * @return {number}
 */
var uniquePathsWithObstacles = function(obstacleGrid) {
    let m = obstacleGrid.length;
    let n = obstacleGrid[0].length

    // set end cell to any value different from blocked cell
    obstacleGrid[m - 1][n - 1] = obstacleGrid[m - 1][n - 1] === 0 ? 2 : 1;

    let arr = obstacleGrid

    for(let y = m - 1; y >= 0; y--) {
      for(let x = n - 1; x >= 0; x--) {
        // shortcut to bypass blocked cell
        if(arr[y][x] === 1) {
          arr[y][x] = 0 // set blocked cell to 0
          continue
        }
        if(arr[y][x] === 2) {
          arr[y][x] = 1
        }

        // left cell
        if(x + 1 < n) {
          arr[y][x] += arr[y][x + 1]
        }
        // down cell
        if(y + 1 < m) {
          arr[y][x] += arr[y + 1][x]
        }
      }
    }

    return arr[0][0]
};


console.log(uniquePathsWithObstacles([[0,0,0],[0,1,0],[0,0,0]]))
console.log(uniquePathsWithObstacles([[0,0],[0,1]]))
