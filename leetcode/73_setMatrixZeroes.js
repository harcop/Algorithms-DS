// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.

// You must do it in place.

/**
 * @param {number[][]} matrix
 * @return {void} Do not return anything, modify matrix in-place instead.
 */
var setZeroes = function(matrix) {
    // for zero positions
    let row = [];
    let col = [];
    for(let i = 0; i < matrix.length; i++) {
        for(let j = 0; j < matrix[0].length; j++) {
            let val = matrix[i][j]
            if(val === 0) {
                row.push(i)
                col.push(j)
            }
        }
    }

    if(row.length && col.length) {
        for(let i = 0; i < matrix.length; i++) {
            for(let j = 0; j < matrix[0].length; j++) {
                if(row.includes(i) || col.includes(j)) {
                    matrix[i][j] = 0
                }
            }
        }
    }

    return matrix
};

console.log(setZeroes([[1,1,1],[1,0,1],[1,1,1]]))
console.log(setZeroes([[0,1,2,0],[3,4,5,2],[1,3,1,5]]))
