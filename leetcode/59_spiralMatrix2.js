/**
 * @param {number} n
 * @return {number[][]}
 */
var generateMatrix = function(n) {
    const arr = [];
    for(let i = 1; i <= n * n; i++) {
        arr.push(i)
    }
    const res = [];
    while(arr.length) {
      let nn = arr.splice(0, n)
      res.push(nn)
      if(arr.length) {
        let newArr = [arr.shift()];
        arr.reverse();
        while(newArr.length < n) {
            newArr.unshift(arr.shift())
        }
        res.push(newArr)
      }
    }
    return res
};
console.log(generateMatrix([1, 2, 3, 0, 9, 8, 7, 4, 5], 3))
console.log(generateMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9], 3))
console.log(generateMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 4))
console.log(generateMatrix([1], 1))
