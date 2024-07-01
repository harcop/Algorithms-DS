// You are given an m x n integer matrix matrix with the following two properties:

// Each row is sorted in non-decreasing order.
// The first integer of each row is greater than the last integer of the previous row.
// Given an integer target, return true if target is in matrix or false otherwise.

// You must write a solution in O(log(m * n)) time complexity.


/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
var searchMatrix = function(matrix, target) {
    // find row
    let row = null
    for(let i = 0; i < matrix.length; i++) {
        let val = matrix[i][0]
        if(val === target) {
            return true
        }
        if(val > target) {
            if(i === 0) {
                return false
            }
            break
        }
        row = i;
    }

    for(let i = 0; i < matrix[0].length; i++) {
        let val = matrix[row][i]
        if(val === target) {
            return true
        }
    }
    return false
};

console.log(searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 3))
console.log(searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 13))
console.log(searchMatrix([[1], [3]], 3))
