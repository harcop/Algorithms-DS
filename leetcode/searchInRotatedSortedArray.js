/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
// must be o(log n)
const search = function(nums, target) {
    let j = nums.length - 1;
    for(let i = 0; i <= nums.length / 2; i++) {
        if(nums[i] === target) {
            return i
        }
        if(nums[j] === target) {
            return j
        }
        j--
    }
    return -1
};

console.log(search([4,5,6,7,0,1,2], 0))
console.log(search([4,5,6,7,0,1,2], 3))
console.log(search([1], 0))
