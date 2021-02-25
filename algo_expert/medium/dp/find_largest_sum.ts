function ls1(arr: number[]): number {
    let _a = [0,0,0];
    for (let i = 0; i<arr.length; i++) {
        if (arr[i] > _a[2]) {
            _a[0] = _a[1];
            _a[1] = _a[2];
            _a[2] = arr[i];
        }
        else if (arr[i] > _a[1]) {
            _a[0] = _a[1];
            _a[1] = arr[i];
        }
        else if (arr[i] > _a[0]) {
            _a[0] = arr[i];
        }
    }
    let _m = _a.reduce((a, b) => a+b);
    return _m;
}

function ls(arr: number[]): number {
    arr.sort((a,b) => a-b);
    let _a = arr.splice(arr.length-3);
    let _m = _a.reduce((a, b) => a+b);
    return _m;
}

console.log(ls([7,5,10,9,7,14]))