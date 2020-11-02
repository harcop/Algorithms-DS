const arr = [45, 61, 71, 72, 73, 0, 1, 21, 33, 45];
const k = 61;

// time O(n) && space O(1);
function sb () {
    let lp = 0; 
    let rp = arr.length -1;
    let idx = -1;
    while(lp < rp) {
        let el = arr[lp];
        let er = arr[rp];
        if (el === k) {
            idx = lp;
            break;
        }
        if (er === k) {
            idx = rp;
            break;
        }
        if (el > k) {
            lp += 1;
        } 
        if (er > k) {
            rp -= 1;
        }else {
            lp += 1;
            rp -= 1;
        }
    }
    return idx;
}

console.log(sb())