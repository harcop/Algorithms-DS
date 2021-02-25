const arr = [8,4,2,1,3,6,7,9,5];

function mr () {
    const _a = Array(arr.length);
    _a.fill(1,0,arr.length);
    console.log(_a);
    for(let i =1; i<arr.length; i++) {
        if (arr[i] > arr[i-1]) {
            _a[i] = _a[i-1] + 1;
        }
    }
    console.log(_a);
    for(let i = arr.length-1; i>=0; i--) {
        if (arr[i] > arr[i+1]) {
            _a[i] = Math.max(_a[i], _a[i+1] + 1);
        }
    }
    console.log(_a);
    return _a;
}

console.log(mr());