/**
 * @param {number[]} nums
 * @return {number}
 */
const jump = function(nums) {

  let max = 0;
  
  for (let i = 0; i < nums.length - 1; i++) {
    max = Math.max(max, i + nums[i]);

    if(i === max && max >= nums[i] + max) {
      return false
    }
    
  }

  return max >= nums.length - 1;
};

console.log(jump([1]))
console.log(jump([1, 1]))
console.log(jump([3,2,1]))
console.log(jump([2,1]))
console.log(jump([0]))
console.log(jump([1,2]))
console.log(jump([2,3,1,1,4]))
console.log(jump([2,3,0,1,4]))
console.log(jump([2,2,0,1,4]))
console.log(jump([3,2,1,0,4]))
console.log(jump([1,0,1,0]))
console.log(jump([0,2,3]))
