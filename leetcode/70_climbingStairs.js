// You are climbing a staircase. It takes n steps to reach the top.

// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

// Example 1:

// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
// Example 2:

// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step


/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    let arr = [[1]]
    let temp = ['1']
    if(n === 1) {
      return 1
    }
    let j = 2;
    while(j <= n) {
      mov(arr, temp)
      j++
    }
    console.log(arr)
    return arr.length - 1
};

function mov(arr, temp) {
  let tempArr = [...arr]
  for(let ele of tempArr) {
    arr.push([1, ...ele])
    console.log(arr)
    temp.push([1, ...ele].join(''))

    let jj = temp[0]
    if(!temp.includes(jj)) {
      arr.push([...ele, 1])
      temp.push([...ele, 1].join(''))
    }
    for(let i = 0; i < ele.length; i++) {
      let tempEle = [...ele];
      if(tempEle[i] === 1) {
        tempEle[i] += 1;
        jj = [...tempEle].join('')
        console.log(jj)
        console.log(temp)
        if(!temp.includes(jj)) {
          console.log(temp)
          arr.push(tempEle)
        }
      }
    }
  }
}

// console.log(climbStairs(1))
console.log(climbStairs(2))
// console.log(climbStairs(3))
