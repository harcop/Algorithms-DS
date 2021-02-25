const arr = [2,1,2,2,2,3,4,2];
const k = 2;

// time O(n) && space O(1)
function me(arr, k) {
    let lp = 0;
    let rp = arr.length -1;
    while(lp < rp) {
        if (arr[lp] === k && arr[rp] === k) {
            rp--;
        }
        else if (arr[lp] === k) {
            console.log("Asd");
            [arr[lp], arr[rp]] = [arr[rp], arr[lp]];
            lp++;
            rp--;
        }
        else {
            lp++;
        }
    }
    return arr;
}

console.log(me(arr, k))