const arr = [8, 5, 2, 9, 5, 6, 3];


// time O(n^2) && space O(1)
function ss() {
    for (let i=0; i<arr.length; i++) {
        let m = i;
        isSwap = false;
        for (let j=i+1; j<arr.length; j++) {
            if (arr[m] > arr[j]) {
                m = j;
                isSwap = true;
                continue;
            }
        }
        if (isSwap) {
            swapper(i,m);
        }
    }
    return arr;
}

function swapper(m,i) {
    [arr[m], arr[i]] = [arr[i], arr[m]];
}

console.log(ss(arr));