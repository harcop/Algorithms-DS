const arr = [0,1,21,33,45,46,61,71,72,73];
const f = 61;
function bs(arr) {
    let lp = 0;
    let rp = arr.length-1;

    let m = 0;
    while(rp >= lp) {
        m = Math.floor((rp+lp)/2);
        if (arr[m] === f) {
            return m;
        }
        else if (arr[m] > f) {
            rp = m-1;
        }
        else if (arr[m] < f) {
            lp = m+1;
        }
        console.log(m);
    }
    return false;
}

console.log(bs(arr));