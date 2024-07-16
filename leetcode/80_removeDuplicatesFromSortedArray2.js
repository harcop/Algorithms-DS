/**
 * @param {number[]} nums
 * @return {number}
 */
var removeDuplicates = function(nums) {
    let seen = {}
    let n = nums.length
    for(let i = 0; i < n; i++) {
      if(nums[i] || nums[i] === 0) {
        if(seen[nums[i]] >= 2) {
            nums.splice(i, 1)
            i--
        }else {
            seen[nums[i]] = (seen[nums[i]] || 0) + 1
        }
      }
    }
    return nums;
};

console.log(removeDuplicates([1,1,1,2,2,3]))
console.log(removeDuplicates([0,0,1,1,1,1,2,3,3]))
console.log(removeDuplicates([0,0,0,0,0]))
