function composeRanges(nums: number[]): string[] {
    let _p = [];
    let _sp = [];
    let _f = [];
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] - nums[i-1] > 1) {
            _p.push(_sp);
            _sp = [];
        }
        _sp.push(nums[i]);
    }
    _p.push(_sp);

    _p.forEach(arr => {
        if (arr.length > 1) {
            _f.push(`${arr[0]}->${arr[arr.length-1]}`)
            return;
        }
            _f.push(`${arr[0]}`);
    });
    return _f;
}


console.log(composeRanges([-1, 0, 1, 2, 6, 7, 9]));
console.log(composeRanges([-1, 0, 1, 2, 6, 7, 8]));