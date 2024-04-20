/**
 * @param {number[][]} matrix
 * @return {void} Do not return anything, modify matrix in-place instead.
 */
const rotate = function(matrix) {
    let innerLength = matrix[0].length;
    for(let i = 1; i < innerLength; i++) {
        for(let j = 0; j < innerLength; j++) {
            let ele = matrix[i][j];
            matrix[0][j] = `${ele}-${matrix[0][j]}`
        }
    }
    return matrix.splice(0, 1).map(ele => ele.map(item => item.split('-').map(s => Number(s))))[0]
};

console.log(rotate([[1,2,3],[4,5,6],[7,8,9]]))
console.log(rotate([[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]))
