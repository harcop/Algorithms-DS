/**
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
  let sum = 0,
  max = -Infinity;
  if (nums.length === 1) return nums[0];
  for (let i = 0; i < nums.length; i++) {
    sum += nums[i];
    max = Math.max(max, sum);

    if (sum < 0) {
      sum = 0;
    }
  }
  return max;
};

console.log(maxSubArray([-2,1,-3,4,-1,2,1,-5,4]))
console.log(maxSubArray([-2,1,-3]))
console.log(maxSubArray([-1,0,-2]))
