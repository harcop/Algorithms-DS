function arrayPreviousLess(arr: number[]): number[] {
    let _min = -1;
    for(let i = 0; i < arr.length; i++) {
        if (arr[i] > _min) {
            let temp = arr[i];
            arr[i] = _min
            _min = temp;
            continue
        }
        _min = arr[i];
        arr[i] = -1;
    }
    return arr;
}

console.log(arrayPreviousLess([3, 5, 2, 4, 5]));