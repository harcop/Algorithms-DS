function avoidObstacles(arr: number[]): number {
    arr.sort((a,b) => a-b);
    // let _max = 0;
    // let pos = 0;
    // for(let i = 0; i< arr.length-1; i++) {
    //     let _d = arr[i+1] - arr[i]
    //     if (_d > 1) {
    //         let _m = arr[i] + _d-1;
    //         if (_m % _max !== 0) {
    //             _max = _m;
    //         }
    //     } 
    // }
    // return _max;
    let _max = arr[arr.length-1];
    for(let i = 1; i <= _max+1; i++) {
        if (arr.every((ele) => ele % i !== 0)) {
            return i;
        }
    }
}

console.log(avoidObstacles([5, 3, 6, 7, 9]));
console.log(avoidObstacles([5, 8, 19]));