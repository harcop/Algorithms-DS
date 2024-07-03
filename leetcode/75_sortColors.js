/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var sortColors = function(nums) {
  let zero = [];
  let one = [];
  let two = [];
  for(let val of nums) {
      if(val === 0) {
        zero.push(val)
      }
      if(val === 1) {
        one.push(val)
      }
      if(val === 2) {
        two.push(val)
      }
  }
  let nn = [...zero, ...one, ...two];
  for(let i = 0; i < nums.length; i++) {
    nums[i] = nn[i]
  }
  return nums;
};

// could use pointer swap
// loop swaping

console.log(sortColors([2, 1, 0]))
console.log(sortColors([2,0,2,1,1,0]))
