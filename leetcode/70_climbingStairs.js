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
    if(n === 1) {
      return 1
    }
    let j = 2;
    while(j <= n) {
      mov(arr)
      j++
    }
    let nn = [];
    for(let a of arr) {
      const cc = a.reduce((el, cv) => el + cv)
      if(cc === n && !nn.includes(a.join(''))) {
        nn.push(a.join(''))
      }
    }
    console.log(nn)
    return nn.length
};

function mov(arr) {
  let tempArr = [...arr]
  for(let ele of tempArr) {
    arr.push([1, ...ele])
    arr.push([...ele, 1])
    for(let i = 0; i < ele.length; i++) {
      let tempEle = [...ele];
      if(tempEle[i] === 1) {
        tempEle[i] += 1;
        arr.push(tempEle)
      }
    }
  }
}

// use fibonacci sequence
function fib(n) {
  if(n <= 1) {
    return 1
  }
  return fib(n-1) + fib(n-2)
}

// use fibonacci sequence memoization
function fibMem(n) {
  if(n <= 1) {
    return 1
  }
  let arr = [1, 1]
  let i = 2;
  while(i <= n) {
    arr.push(arr[0]+arr[1]);
    arr.shift()
    i++
  }
  return arr[1]
}

console.log(fibMem(1))
console.log(fibMem(2))
console.log(fibMem(3))
console.log(fibMem(4))
console.log(fibMem(5))

// console.log(climbStairs(1))
// console.log(climbStairs(2))
// console.log(climbStairs(3))
// console.log(climbStairs(4))
console.log(climbStairs(5))
console.log(climbStairs(6))
