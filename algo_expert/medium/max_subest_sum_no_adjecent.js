const arr = [7,10,12,7,9,14];

function ms1(arr) {
    arr.sort((a,b) => a-b);

    let i = 0;
    let sum = 0;
    if (arr.length % 2 === 0) {
        i = 1;
    }
    else {
        i = 0;
    }
    while (i < arr.length) {
        sum += arr[i];
        i += 2
    }
    return arr;
}

//time O(n) && space O(1)
function ms(arr) {
    let _ma = [arr[0]];
    _ma[1] = max(arr[0], arr[1]);
    for (let i=2;i<arr.length; i++) {
        let _m = max(_ma[1], _ma[0]+arr[i]);
        _ma[0] = _ma[1];
        _ma[1] = _m;
    }
    return _ma[1];
}

function max(a, b) {
    if (a>b) {
        return a;
    }
    return b;
}

console.log(ms(arr));