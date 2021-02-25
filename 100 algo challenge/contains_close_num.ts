function containsCloseNums(nums: number[], k: number): boolean {
    let _p = [];
    let _s = [];
    for (let i = 0; i<nums.length; i++) {
        if (_p.includes(nums[i])) {
            if (Math.abs(_s[_p.indexOf(nums[i])] - i) <= k) {
                return true
            }
        }
        _p.push(nums[i]);
        _s.push(i);
    }
    return false;
}

console.log(containsCloseNums([0, 1, 2, 3, 5, 2], 3));
console.log(containsCloseNums([0, 1, 2, 3, 5, 2], 2));