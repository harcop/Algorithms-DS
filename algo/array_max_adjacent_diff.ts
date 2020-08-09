function arrayMaximalAdjacentDifference(arr: number[]): number {
    let _max = 0;
    for (let i = 0; i< arr.length-1; i++) {
        let diff = arr[i] - arr[i+1];
        if (Math.abs(diff) > _max) {
            _max = Math.abs(diff)
        }
    }
    return _max;
}

console.log(arrayMaximalAdjacentDifference([2, 4, 1, 0]));
console.log(arrayMaximalAdjacentDifference([2, 4, 5, 0]));