const arr = [
    [2,3,1],
    [2,1,2],
    [3,2,3],
    [2,3,4],
    [4,4,5],
    [2,2,8]
]


//time O(n^2) && space O(1)
function ds(arr) {
    let _md = [];
    _md.push(arr[0][2]);
    for(let i=1; i<arr.length; i++) {
        _md.push(arr[i][2]);
        let _mh = 0;
        for(let j = 0; j < i; j++) {
            if (arr[i][0] > arr[j][0] && arr[i][1] > arr[j][1] && arr[i][2] > arr[j][2] && _md[j] >= _mh) {
                _mh = _md[j];
            }
        }
        _md[i] += _mh
    }
    return Math.max(..._md);
}

console.log(ds(arr));