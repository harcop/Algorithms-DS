/**
 * @param {number[]} nums
 * @return {number}
 */
const jump = function(nums) {
  if (nums.length === 1) return 0;

  let prev = 0, max = 0, jump = 0;
  
  for (let i = 0; i < nums.length - 1; i++) {
    console.log(nums[i])
    max = Math.max(max, i + nums[i]);

    if (i === prev) {
        jump++;
        prev = max;
    }
    
  }

  return jump;
};

console.log(jump([1]))
console.log(jump([3,2,1]))
console.log(jump([2,1]))
console.log(jump([0]))
console.log(jump([1,2]))
console.log(jump([2,3,1,1,4]))
console.log(jump([2,3,0,1,4]))
console.log(jump([2,2,0,1,4]))
