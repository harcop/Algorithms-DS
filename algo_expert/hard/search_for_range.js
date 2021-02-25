const arr = [0,1,21,33,45,45,45,45,45,45,61,71,73];
const k = 45;

// time O(log(n)) && space O(1)
function sr() {
    let lp = 0;
    let rp = arr.length-1
    let _r = [-1,-1];
    while (lp < rp) {
        if (arr[lp] === k && _r[0] === -1) {
            _r[0] = lp;
        }
        if (arr[rp] === k && _r[1] === -1) {
            _r[1] = rp;
        }
        if (_r[0] !== -1 && _r[1] !== -1) {
            break;
        }
        lp += 1;
        rp -= 1;
    }
    return _r;
}

console.log(sr());