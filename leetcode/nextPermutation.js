
// Approach

// Find the first decreasing index moving from end to start
// E.g. [7, 2, 3, 1, 5, 4, 3, 2, 0] num 1 is the first decreasing index going from the end backwards
// Swap num 1 with the next large num to its right which is 2
// [7, 2, 3, 2, 5, 4, 3, 1, 0]
// Reverse/sort nums to the right
// [7, 2, 3, 2, 0, 1, 3, 4, 5]
// If there is no next permutation reverse the array


const nextPermutation = (nums) => {
    for(let i = nums.length - 2; i >= 0; i--) {
      if(nums[i] < nums[i + 1]) {
        for(let j = nums.length - 1; j >= 0; j--) {
          if(nums[i] < nums[j]) {
            // swap
            swap(nums, i, j)

            // sort everything after i;
            reverse(nums, i);
            return nums
          }
        }
      }
    }
    return nums.reverse()
};

function swap(nums, i, j) {
  const temp = nums[i];
  nums[i] = nums[j]
  nums[j] = temp;
}

function reverse(nums, i) {
  for(let k = i + 1; k < nums.length; k++) {
    for(let l = k + 1; l < nums.length; l++) {
      if(nums[k] > nums[l]) {
        swap(nums, k, l)
      }
    }
  }
}

console.log(nextPermutation([4,2,0,2,3,2,0]))
console.log(nextPermutation([1, 2, 3]))
console.log(nextPermutation([1, 3, 2]))
console.log(nextPermutation([3, 2, 1]))
console.log(nextPermutation([2, 3, 1]))
