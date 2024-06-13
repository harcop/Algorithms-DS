/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
var uniquePaths = function(m, n) {
    // create a m x n grid
    const arr = []
    for(let i = 0; i < m; i ++) {
        const narr = [] // new array
        for(let j = 0; j < n; j++) {
            narr.push(0)
        }
        arr.push(narr)
    }

    arr[m - 1][n - 1] = 1; 

    for(let y = m - 1; y >= 0; y--) {
      for(let x = n - 1; x >= 0; x--) {
        if(x + 1 < n) {
          arr[y][x] += arr[y][x + 1]
        }
        if(y + 1 < m) {
          arr[y][x] += arr[y + 1][x]
        }
      }
    }

    return arr[0][0]
};

// you would have to compute the max grid from the target backward to the starting point
// for a 3 x 2 grid
// [
//   [3, 1]
//   [2, 1]
//   [1, 1]
// ]


console.log(uniquePaths(3, 7))
