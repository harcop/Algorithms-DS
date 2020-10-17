
const arr = [0,8,0,0,5,0,0,10,0,0,1,1,0,3];

// time O(n) && space O(1);
function wa() {
    let lp = 0;
    let rp = arr.length-1;
    let _m = 0;
    let _wa = 0;
    while (lp < rp) {
        if (arr[lp] > arr[rp]) {
            if (arr[rp] < _m) {
                _wa += _m - arr[rp];
            }else {
                _m = arr[rp];
            }
            rp -= 1;
        }
        else if (arr[rp] > arr[lp]) {
            if (arr[lp] < _m) {
                _wa += _m - arr[lp];
            }else {
                _m = arr[lp];
            }
            lp += 1;
        }
    }
    return _wa;
}

console.log(wa());