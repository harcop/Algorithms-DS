function arrayMaxConsecutiveSum(arr: number[], k: number): number {
    let _max = 0;
    for(let i = 0; i < arr.length-k+1; i++) {
        let _sum = arr[i];
        for (let j = 0; j<k-1; j++) {
            _sum += arr[i+j+1] //for number of k consecutive
        }
        if (_sum > _max) {
            _max = _sum
        }
        _sum = 0;
    }
    return _max;
}

console.log(arrayMaxConsecutiveSum([2, 3, 5, 1, 6], 2));
console.log(arrayMaxConsecutiveSum([4, 3, 8, 2, 1], 3));