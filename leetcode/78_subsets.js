// Given an integer array nums of unique elements, return all possible 
// subsets
//  (the power set).

// The solution set must not contain duplicate subsets. Return the solution in any order.

 

// Example 1:

// Input: nums = [1,2,3]
// Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsets = function(nums) {
    const results = [[]];
    for(let i = 0; i < nums.length; i++) {
      console.log(i)
      let nn = [];
      let nnn = []
      for(let j = 0; j < results.length; j++) {
        nn = [...results[j]]
        nn.push(nums[i]);
        nnn.push(nn)
      }
      results.push(...nnn)
    }
    return results;
};

console.log(subsets([1,2,3]))
console.log(subsets([0]))
