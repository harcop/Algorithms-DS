// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.

// If target is not found in the array, return [-1, -1].

// You must write an algorithm with O(log n) runtime complexity.

const searchRange = function(nums, target) {
    let left = -1;
    let right = -1;

    let lp = 0;
    let rp = nums.length - 1;
    for(const _element of nums) {
        if(nums[lp] === target && left === -1) {
            left = lp;
        } else if(left === -1) {
            lp += 1;
        }

        if(nums[rp] === target && right === -1) {
            right = rp;
        } else if(right === -1) {
            rp -= 1;
        }
    }

    return [left, right]
};
