/**
 * @param {number[][]} matrix
 * @return {void} Do not return anything, modify matrix in-place instead.
 */
const rotate = function(matrix) {
    let innerLength = matrix[0].length;
    let stage = innerLength;
    let swap = [];
    for(let i = 0; i < innerLength; i++) {
      stage--
      for(let j = 0; j < innerLength; j++) {
          if(!swap.includes(`${i}-${j}`)) {
            let ele = matrix[i][j];
            let temp = matrix[j][stage];
            matrix[i][j] = temp;
            matrix[j][stage] = ele;
            swap.push(`${j}-${stage}`)
          }
      }
    }
    return matrix
};

console.log(rotate([[1,2,3],[4,5,6],[7,8,9]]))
console.log(rotate([[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]))
